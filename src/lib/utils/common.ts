import { invoke } from '@tauri-apps/api/core';
import type { z } from 'zod';
import moment from 'moment';

import type {
    BranchOwnerDetailsType, InvoiceSummaryType,
    VendorDetailsType, StateListType,
    InvoiceDetailsType,
    ProductDetailsType, TaxDetailsType
} from '$lib/utils/schemas';
import type { InvoiceDetailsSchema } from '$lib/utils/schemas';
import { APP_UI_STATE, PDF_SERVICE_STATE } from "$lib/app/state.svelte";

const AMOUNT_FORMATTER = new Intl.NumberFormat("en-in", {
    style: "currency",
    currency: "INR"
})

export enum Mode {
    ADD,
    UPDATE,
    DELETE
}

type LogType = "info" | "error";

export function makeReadable(str: string) {
    if (str && str.indexOf("_") > -1) {
        return str.split("_").map(capitalize).join(" ");
    }

    if (str && str.indexOf("") > -1) {
        return str.split(" ").map(capitalize).join("");
    }

    return str ? capitalize(str) : '--';
}

export function capitalize(str: string) {
    if (str) {
        return str[0].toUpperCase() + str.slice(1)
    }

    return "";
}

export function dateToISOString(data: { [key: string]: any }) {
    Object.keys(data).forEach((key) => {
        const value = data[key];
        if (value instanceof Date) {
            data[key] = value.toISOString().replace("Z", "");
        }
    })
}

export function dateToInputPattern(data: { [key: string]: any }, ...fieldNames: string[]) {
    fieldNames.forEach((key) => {
        const value = data[key];
        const parsedDate = tryParseDate(value);
        if (parsedDate != null) {
            data[key] = getFormattedDateString(parsedDate);
        }
    })
}

export function getFormattedDateString(date: Date): string {
    let month: any = date.getMonth() + 1;
    if (month < 10) month = `0${month}`;
    
    let dayOfMonth: any = date.getDate();
    if (dayOfMonth < 10) dayOfMonth = `0${dayOfMonth}`;
    
    return `${date.getFullYear()}-${month}-${dayOfMonth}`;
}

function tryParseDate(date: any): Date | null {
    try {
        if (date instanceof Date) {
            return date;
        }

        return new Date(date);
    } catch (ex) {
        console.error(`Error parsing date string: ${date}`)
    }

    return null;
}

export function convertPropertiesArrayToBlob(data: { [key: string]: any }, type: string, ...fieldNames: string[]) {
    fieldNames.forEach((key) => {
        data[key] = arrayToBlob(data[key] as Array<number>, type);
    })
}

export const arrayToBlob = (array: Array<number>, type?: string): Blob => {
    const value = new Uint8Array(array);
    return new Blob([value], { type });
}

export function removeEmptyFields(data: any) {
    const result: { [key: string]: any } = {}

    for (const [key, value] of Object.entries(data)) {
        if (value) {
            result[key] = value;
        }
    }

    return result;
};

export function debounceWrapper(callback: Function, wait: number) {
    let timeout: ReturnType<typeof setTimeout>;
    
    return (...args: any[]) => {
        clearTimeout(timeout);
        timeout = setTimeout(() => callback(...args), wait);
    }
};

export function formatDate(date: Date): string {
    return moment(date).format(APP_UI_STATE.dateFormat);
}

export function formatAmount(amount: number): string {
    return AMOUNT_FORMATTER.format(amount);
}

export async function searchStates(query: string) {
    return await invoke("find_states", { query })
}

export const blobToBase64 = (blob: Blob, trim?: boolean): Promise<string> => {
    return new Promise((resolve, _) => {
        const reader = new FileReader();
        
        reader.onloadend = () => {
            let resultStr = reader.result as string;
            const index = resultStr.indexOf(",");

            if (trim && index != -1) {
                resultStr = resultStr.substring(index+1);
            }

            resolve(resultStr);
        }

        reader.readAsDataURL(new Blob([blob], { type: "image/*" }));
    })
}

export const blobToUint8Array = async (blob: Blob): Promise<Uint8Array> => {
    let buffer = await blob.arrayBuffer();
    return new Uint8Array(buffer);
};

export const time = (millis: number): Promise<void> => {
    return new Promise((resolve, _) => {
        setTimeout(resolve, millis);
    })
}

export const log = (msg: string, logType: LogType = "info"): Promise<any> => {
    return invoke("log", { msg, logType })
}

export const loadFontForGoService = () => {
    const goServiceURL = getGoServiceURL();
    const url = `${goServiceURL}/loadFont/Roboto/ttf`;

    fetch(url, { method: "GET" })
        .then(async (response) => {
            let resTxt = await response.text();
            log(`Load font response: ${resTxt}\n`);
        })
        .catch(console.error)
}

export const getInvoicePDF = (payload: { [key: string]: any }): Promise<Blob> => {
    const goServiceURL = getGoServiceURL();
    const pdfUrl = `${goServiceURL}/invoicePdf`;

    return new Promise((resolve, reject) => {
        sendRequest(pdfUrl, "POST", JSON.stringify(payload))
            .then((response: Response) => {
                if (response.ok) {
                    response.blob().then(resolve);
                } else {
                    response.json().then(reject);
                }
            })
            .catch(reject);
    });
}

const getGoServiceURL = () => {
    return `http://127.0.0.1:${PDF_SERVICE_STATE.port}`;
}

const sendRequest = async (url: string, method?: string, body?: BodyInit | null): Promise<Response> => {
    const requestInit: RequestInit = {};
    if (method) {
        requestInit.method = method;
    }
    if (body) {
        requestInit.body = body;
    }

    return fetch(url, requestInit);
};

export const buildInvoiceData = async (
    bod: BranchOwnerDetailsType, summary: InvoiceSummaryType,
    vendor: VendorDetailsType, vendorState: StateListType,
    details: z.infer<typeof InvoiceDetailsSchema>[], productsMap: { [key: string]: ProductDetailsType },
    taxDetail: TaxDetailsType
): Promise<{ [key: string]: any }> => {
    const invoiceData = {
        branch_owner: await buildBranchOwnerData(bod),
        invoice_summary: {
            invoice_number: summary.invoice_number,
            invoice_date: summary.invoice_date,
            place_of_supply: `${vendorState.tin}-${vendorState.state_name}`,
            financial_year: summary.financial_year
        },
        vendor: {
            vendor_name: vendor.vendor_name,
            address: {
                address_lines: [
                    vendor.address1, vendor.address2
                ],
                state: vendor.address4,
                pin_code: parseInt(vendor.pin_code)
            },
            phone_number: vendor.phone_number,
            email: vendor.email_id,
            gstin: vendor.vgstin
        },
        transactions: buildTransactions(details.map((detail) => Object.assign(detail, productsMap[detail.product_id]))),
        total: {
            product_name: "Total",
            quantity: 0,
            no_of_bags: 0,
            amount: parseFloat(summary.amount)
        },
        other_charges: [{
            name: "Packaging Charges",
            amount: parseFloat(summary.pkg_charges || "0")
        }],
        taxable_amount: {
            product_name: "Taxable Amount",
            amount: parseFloat(summary.taxable_amount)
        },
        gst_charges: buildGSTCharges(summary, taxDetail),
        all_total: {
            product_name: "Total (Incl Tax + Charges)",
            amount: parseFloat(summary.total_amount)
        },
        notes: [
            "Thank you for doing business with us",
        ],
        terms_and_conditions: [
            "Goods once sold can not be taken back or exchanged",
            "Payment must be done in 2 days",
            "Subject to local Jurisdiction"
        ]
    }

    return invoiceData;
}

const buildGSTCharges = (summary: InvoiceSummaryType, taxDetail: TaxDetailsType) => {
    let charges = [];

    if (parseFloat(taxDetail.tax_rate1) > 0) {
        charges.push({
            name: taxDetail.tax_name1,
            percent: parseFloat(taxDetail.tax_rate1),
            amount: parseFloat(summary.tax1)
        })
    }

    if (parseFloat(taxDetail.tax_rate2) > 0) {
        charges.push({
            name: taxDetail.tax_name2,
            percent: parseFloat(taxDetail.tax_rate2),
            amount: parseFloat(summary.tax2)
        })
    }

    if (parseFloat(taxDetail.tax_rate3) > 0) {
        charges.push({
            name: taxDetail.tax_name3,
            percent: parseFloat(taxDetail.tax_rate3),
            amount: parseFloat(summary.tax3)
        })
    }

    return charges;
}

const buildTransactions = (details: (InvoiceDetailsType & ProductDetailsType)[]): { [key: string]: any }[] => {
    return details.map((detail: InvoiceDetailsType & ProductDetailsType) => {
        return {
            product_name: detail.short_name,
            hsn_sac: detail.hsn_sac,
            unit: detail.unit,
            quantity: detail.quantity,
            price_per_unit: parseFloat(detail.price_per_unit),
            no_of_bags: detail.no_of_bags,
            amount: parseFloat(detail.amount)
        }
    });
}

const buildBranchOwnerData = async (bod: BranchOwnerDetailsType) => {
    const icon = arrayToBlob(bod.icon as unknown as Array<number>, "image/*");
    const signatory = arrayToBlob(bod.signatory as unknown as Array<number>, "image/*");

    const data: { [key: string]: any } = {
        icon: await blobToBase64(icon, true),
        legal_business_name: bod.legal_business_name,
        first_name: bod.first_name,
        middle_name: bod.middle_name,
        last_name: bod.last_name,
        gstin: bod.gstin,
        address: {
            address_lines: [
                bod.address1,
                bod.address2,
            ],
            state: bod.address4,
            pin_code: parseInt(bod.pin_code)
        },
        phone_number: bod.phone_number,
        email: bod.email_id,
        upi_id: bod.upi_id,
        bank_name:bod.bank_name,
        branch_name: bod.branch_name,
        account_number: bod.account_number,
        ifsc_code: bod.ifsc_code,
        account_holder_name: bod.account_holder_name,
        signatory: await blobToBase64(signatory, true)
    };

    return data;
}
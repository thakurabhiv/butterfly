/**
 * This is where all schema for forms is defined.
 * Note: For all fields input type will be string only, unless it is a File type
 */

import { z } from 'zod';
import * as regexp from '$lib/utils/regexprs';

// get these values from database
const taxTypes: [string, ...string[]] = ["IGST", "CGST", "SGST"];
const maximumTaxPer: number = 28;

const getInitialObject = <T extends z.ZodObject<any>>(schema: T, valueMap: { [key: string]: string } = {}): z.input<T> => {
    let initialObject: { [key: string]: any } = {};

    const schemaShape = schema.shape;
    for (let field in schemaShape) initialObject[field] = valueMap[field] || "";

    return initialObject;
}

const ProductDetailsSchema = z.object({
    product_id: z.coerce.string().pipe(z.coerce.number()).optional(),
    short_name: z.string()
        .min(3, "Too short (Min: 3)")
        .max(100, "Too long (Max: 100)"),
    descr: z.string()
        .min(10, "Too short (Min: 10)")
        .max(200, "Too long (Max: 200)")
        .optional(),
    hsn_sac: z.string()
        .min(1, "Enter HSN number")
        .max(10, "Too long (Max: 10)"),
    price: z.coerce.string()
        .min(1, "Enter price")
        .pipe(z.coerce.number().positive("Price should be positive")),
    is_deleted: z.coerce.string().transform(val => !!val && val.toLowerCase() == "true")
});

const TaxDetailsSchema = z.object({
    tax_id: z.coerce.string().pipe(z.coerce.number()).optional(),
    financial_year: z.string()
        .regex(regexp.FINANCIAL_YEAR_REGEXP, "Select financial year"),
    tax_name1: z.enum(taxTypes),
    tax_name2: z.enum(taxTypes),
    tax_name3: z.enum(taxTypes),
    tax_rate1: z.coerce.string()
        .nonempty("Enter tax rate")
        .min(1, "Enter tax rate")
        .pipe(z.coerce.number()
            .min(1, "Value is too low (Min: 1)")
            .max(maximumTaxPer, `Value is too high (Max: ${maximumTaxPer})`)
        ),
    tax_rate2: z.coerce.string()
        .nonempty("Enter tax rate")
        .min(1, "Enter tax rate")
        .pipe(z.coerce.number()
            .min(1, "Value is too low (Min: 1)")
            .max(maximumTaxPer, `Value is too high (Max: ${maximumTaxPer})`)
        ),
    tax_rate3: z.coerce.string()
        .nonempty("Enter tax rate")
        .min(1, "Enter tax rate")
        .pipe(z.coerce.number()
            .min(1, "Value is too low (Min: 1)")
            .max(maximumTaxPer, `Value is too high (Max: ${maximumTaxPer})`)
        ),
    is_deleted: z.coerce.string().transform(val => !!val && val.toLowerCase() == "true")
});

const BranchOwnerDetailsSchema = z.object({
    id: z.coerce.string().pipe(z.coerce.number()).optional(),
    branch_id: z.coerce.string().pipe(z.coerce.number()).optional(),
    first_name: z.string()
        .min(2, "Too short (Min 2 characters)")
        .max(100, "Too large (Max 100 characters)"),
    middle_name: z.string()
        .min(3, "Too short (Min 3 characters)")
        .max(100, "Too large (Max 100 characters)").optional(),
    last_name: z.string()
        .min(3, "Too short (Min 3 characters)")
        .max(100, "Too large (Max 100 characters)"),
    legal_business_name: z.string()
        .min(5, "Too short (Min 5 characters)")
        .max(100, "Too large (Max 100 characters)"),
    birth_date: z.coerce.string().pipe(z.coerce.date()),
    address1: z.string()
        .min(3, "Too short (Min 3 characters)")
        .max(100, "Too Long (Max 100 characters)"),
    address2: z.string()
        .min(3, "Too short (Min 3 characters)")
        .max(100, "Too Long (Max 100 characters)")
        .optional(),
    address4: z.string() // state
        .min(3, "Too short (Min 3 characters)")
        .max(100, "Too Long (Max 100 characters)"),
    state_id: z.coerce.string({ required_error: "Select state" }).pipe(z.coerce.number()),
    pin_code: z.string().trim()
        .length(6, "Shoule be 6 characters"),
    phone_number: z.string()
        .regex(regexp.PHONE_NUMBER_REGEXP, "Invalid phone number"),
    email_id: z.string().email("Enter valid email"),
    gstin: z.string()
        .regex(regexp.GSTIN_REGEXP, "Enter valid GSTIN"),
    fssai: z.string()
        .regex(regexp.FSSAI_REGEXP, "Enter valid FSSAI")
        .optional(),
    account_holder_name: z.string()
        .min(2, "Too short (Min 2 characters)")
        .max(100, "Too Long (Max 100 characters)"),
    bank_name: z.string()
        .min(4, "Too short (Min 4 characters)")
        .max(100, "Too Long (Max 100 characters)"),
    branch_name: z.string()
        .min(4, "Too short (Min 4 characters)")
        .max(100, "Too Long (Max 100 characters)"),
    account_number: z.string()
        .regex(regexp.BANK_ACC_NUMBER_REGEXP, "Enter valid account number"),
    ifsc_code: z.string()
        .regex(regexp.IFSC_REGEXP, "Enter valid IFSC code"),
    upi_id: z.string()
        .regex(regexp.UPI_ID_REGEXP, "Enter valid UPI id")
        .optional(),
    signatory: z.instanceof(File).or(z.instanceof(Blob)).transform(async (file) => {
        let uint8Array = new Uint8Array(await file.arrayBuffer());
        return Array.from(uint8Array);
    }),
    icon: z.instanceof(File).or(z.instanceof(Blob)).transform(async (file) => {
        let uint8Array = new Uint8Array(await file.arrayBuffer());
        return Array.from(uint8Array);
    }),
    is_deleted: z.coerce.string().transform(val => !!val && val.toLowerCase() == "true")
});

const VendorDetailsSchema = z.object({
    vendor_id: z.coerce.string().pipe(z.coerce.number()).optional(),
    vendor_name: z.string({ required_error: "Vendor name required" })
        .min(4, "Too short (Min 2 characters)")
        .max(100, "Too large (Max 100 characters)"),
    address1: z.string({ required_error: "Address required" })
        .min(3, "Too short (Min 3 characters)")
        .max(100, "Too Long (Max 100 characters)"),
    address2: z.string()
        .min(3, "Too short (Min 3 characters)")
        .max(100, "Too Long (Max 100 characters)")
        .optional(),
    address4: z.string({ required_error: "State name required" }) // state
        .min(3, "Too short (Min 3 characters)")
        .max(100, "Too Long (Max 100 characters)"),
    state_id: z.coerce.string({ required_error: "Select state" }).pipe(z.coerce.number()),
    pin_code: z.string({ required_error: "Pin code required" })
        .trim()
        .length(6, "Shoule be 6 characters"),
    phone_number: z.string({ required_error: "Phone number required" })
        .regex(regexp.PHONE_NUMBER_REGEXP, "Invalid phone number"),
    email_id: z.string({ required_error: "Email required" })
        .email("Enter valid email"),
    vgstin: z.string({ required_error: "GSTIN required" })
        .regex(regexp.GSTIN_REGEXP, "Enter valid GSTIN"),
    fssai: z.string()
        .regex(regexp.FSSAI_REGEXP, "Enter valid FSSAI")
        .optional(),
    account_holder_name: z.string({ required_error: "Account holder name required" })
        .min(2, "Too short (Min 2 characters)")
        .max(100, "Too Long (Max 100 characters)"),
    bank_name: z.string({ required_error: "Bank name required" })
        .min(4, "Too short (Min 4 characters)")
        .max(100, "Too Long (Max 100 characters)"),
    branch_name: z.string({ required_error: "Branch name required" })
        .min(4, "Too short (Min 4 characters)")
        .max(100, "Too Long (Max 100 characters)"),
    account_number: z.string({ required_error: "Account number required" })
        .regex(regexp.BANK_ACC_NUMBER_REGEXP, "Enter valid account number"),
    ifsc_code: z.string({ required_error: "IFSC code required" })
        .regex(regexp.IFSC_REGEXP, "Enter valid IFSC code"),
    is_deleted: z.coerce.string().transform(val => !!val && val.toLowerCase() == "true")
});

const InvoiceSummarySchema = z.object({
    invoice_id: z.number().optional(),
    invoice_number: z.string().min(2),
    tax_id: z.coerce.string().pipe(z.coerce.number().positive()),
    invoice_date: z.coerce.string().pipe(z.coerce.date()),
    financial_year: z.string()
        .regex(regexp.FINANCIAL_YEAR_REGEXP, "Select financial year"),
    vendor_id: z.coerce.string().pipe(z.coerce.number().positive()),
    amount: z.coerce.string().pipe(z.coerce.number().positive()),
    pkg_charges: z.coerce.string().pipe(z.coerce.number().positive()),
    taxable_amount: z.coerce.string().pipe(z.coerce.number().positive()),
    tax1: z.coerce.string().pipe(z.coerce.number().gte(0)),
    tax2: z.coerce.string().pipe(z.coerce.number().gte(0)),
    tax3: z.coerce.string().pipe(z.coerce.number().gte(0)),
    total_amount: z.coerce.string().pipe(z.coerce.number().positive()),
    comments: z.string().min(10).max(300).optional()
});

const InvoiceDetailsSchema = z.object({
    id: z.coerce.string().pipe(z.coerce.number().positive()).optional(),
    invoice_id: z.string(),
    product_id: z.coerce.string().pipe(z.coerce.number().positive()),
    quantity: z.coerce.string()
        .pipe(z.coerce.number().gt(0, "Value should be greater than zero")),
    unit: z.coerce.string(),
    price_per_unit: z.coerce.string().pipe(z.coerce.number().positive()),
    no_of_bags: z.coerce.number().gt(0),
    amount: z.coerce.string().pipe(z.coerce.number().positive()),
    tax1: z.coerce.string().pipe(z.coerce.number().positive()),
    tax2: z.coerce.string().pipe(z.coerce.number().positive()),
    tax3: z.coerce.string().pipe(z.coerce.number().positive()),
    total_amount: z.coerce.string().pipe(z.coerce.number().positive()),
    is_deleted: z.coerce.string().transform(val => !!val && val.toLowerCase() == "true")
});

const InvoiceProductSchema = z.object({
    id: z.coerce.string().pipe(z.coerce.number()).optional(),
    invoice_id: z.coerce.string().pipe(z.coerce.number()).optional(),
    product_id: z.coerce.string().pipe(z.coerce.number()).optional(),
    short_name: z.string({
        required_error: "Select product"
    }),
    hsn_sac: z.string()
        .min(1, "Enter HSN number")
        .max(10, "Too long (Max: 10)"),
    price: z.coerce.string()
        .min(1, "Enter price")
        .pipe(z.coerce.number().positive("Price should be positive")),
    quantity: z.coerce.string({
        required_error: "Enter quantity"
    })
        .pipe(z.coerce.number({ invalid_type_error: "Enter quantity" }).positive("Quantity should be positive value")),
    amount: z.coerce.string()
        .pipe(z.coerce.number()),
    no_of_bags: z.coerce.string()
        .pipe(z.coerce.number().gt(0, "Should be greater than zero")).optional()
});

const StateListSchema = z.object({
    id: z.coerce.string().pipe(z.coerce.number()).optional(),
    state_name: z.coerce.string(),
    tin: z.coerce.string().max(2),
    iso: z.coerce.string(),
    capital: z.coerce.string(),
    vehicle_code: z.coerce.string().max(2)
});

type TaxDetailsType = z.input<typeof TaxDetailsSchema>;
type ProductDetailsType = z.input<typeof ProductDetailsSchema>;
type BranchOwnerDetailsType = z.input<typeof BranchOwnerDetailsSchema>;
type VendorDetailsType = z.input<typeof VendorDetailsSchema>;
type InvoiceSummaryType = z.input<typeof InvoiceSummarySchema>;
type InvoiceDetailsType = z.input<typeof InvoiceDetailsSchema>;
type InvoiceProductType = z.input<typeof InvoiceProductSchema>;
type StateListType = z.input<typeof StateListSchema>;

export {
    getInitialObject,
    
    ProductDetailsSchema, type ProductDetailsType,
    TaxDetailsSchema, type TaxDetailsType,
    BranchOwnerDetailsSchema, type BranchOwnerDetailsType,
    VendorDetailsSchema, type VendorDetailsType,
    InvoiceSummarySchema, type InvoiceSummaryType,
    InvoiceDetailsSchema, type InvoiceDetailsType,
    InvoiceProductSchema, type InvoiceProductType,
    StateListSchema, type StateListType,
}
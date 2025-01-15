const PHONE_NUMBER_REGEXP = /((\+*\d{2}\s)*\d+)+/;
const EMAIL_REGEXP = /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/;
const GSTIN_REGEXP = /[0-9]{2}[A-Z]{3}[ABCFGHLJPTF]{1}[A-Z]{1}[0-9]{4}[A-Z]{1}[1-9A-Z]{1}Z[0-9A-Z]{1}/;
const FSSAI_REGEXP = /[0-9]{14}/;
const BANK_ACC_NUMBER_REGEXP = /[0-9]{9,18}/;
const IFSC_REGEXP = /[A-Za-z]{4}0[a-zA-Z0-9]{6}/;
const UPI_ID_REGEXP = /[a-zA-Z0-9.\-_]{2,256}@[a-zA-Z]{2,64}/;
const FINANCIAL_YEAR_REGEXP = /\d{4}-\d{2}/;
const ISO_DATE_REGEXP = /[0-9]{4}-[0-9]{2}-[0-9]{2}(T[0-9]{2}:[0-9]{2}:[0-9]{2})?/;

export {
    PHONE_NUMBER_REGEXP,
    EMAIL_REGEXP,
    GSTIN_REGEXP,
    FSSAI_REGEXP,
    BANK_ACC_NUMBER_REGEXP,
    IFSC_REGEXP,
    UPI_ID_REGEXP,
    FINANCIAL_YEAR_REGEXP,
    ISO_DATE_REGEXP
}
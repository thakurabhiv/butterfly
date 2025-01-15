import * as Forms from '$lib/app/forms'
import KPDFViewer from '$lib/components/custom/KPDFViewer.svelte';

const menuMap = [
    {
        'name': 'Tax Details',
        'component': Forms.TaxDetails
    },
    {
        'name': 'Product Details',
        'component': Forms.ProductDetails
    },
    {
        'name': 'Branch Owner Details',
        'component': Forms.BranchOwnerDetails
    },
    {
        'name': 'Vendor Details',
        'component': Forms.VendorDetails
    },
    {
        'name': 'Sales Invoice',
        'component': Forms.SalesInvoice
    }
]

export {
    menuMap as MenuMap
}
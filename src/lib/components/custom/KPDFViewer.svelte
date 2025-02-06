<script lang="ts" module>
	import * as PDFJS from 'pdfjs-dist';

	// set worker js path
	PDFJS.GlobalWorkerOptions.workerSrc = new URL('pdfjs-dist/build/pdf.worker.mjs', import.meta.url).toString();

	export type KPDFViewerAttributes = {
		name: string,
		src: Blob,
		scaleIncrement?: number,
		onPrintEnd?: Function
	};
</script>
<script lang="ts">
	import { tick, onMount } from 'svelte';

    import { Separator } from '$lib/components/ui/separator';
    import { Input } from '$lib/components/ui/input';
	import { ScrollArea } from '$lib/components/ui/scroll-area';

	import { ZoomIn, ZoomOut, Printer, Download } from 'lucide-svelte';

    import type {
        PDFDocumentProxy, PageViewport
    } from 'pdfjs-dist/types/src/pdf';
    import type { RenderParameters } from 'pdfjs-dist/types/src/display/api';
	import PrintJS from 'print-js';

	import { time, blobToBase64, log } from '$lib/utils/common';

	let {
		name,
		src,
		scaleIncrement = 0.2,
		onPrintEnd = () => {}
	}: KPDFViewerAttributes = $props();

	let pages: string[] = $state([]);
	let canvasArray = $state([] as HTMLCanvasElement[]);
	let scale = $state(1.5);
	let scalePercent = $derived((scale*100).toFixed(0) + "%");
	let pdfDocument: PDFDocumentProxy;

	const startIfReady = async () => {
		if (src && src.size > 0) {
			await time(500);
			start(src);
		}
	}

	const start = (pdfBlob: Blob) => {
		pdfBlob.arrayBuffer()
			.then((buffer: ArrayBuffer) => {
				loadPDF(buffer)
			})
			.catch((reason) => {
				console.log(reason);
			})
	}

	const loadPDF = (buffer: ArrayBuffer) => {
		let loadingTask = PDFJS.getDocument(buffer);
		loadingTask.promise.then((document: PDFDocumentProxy) => {
			pdfDocument = document;
			const noOfPages = pdfDocument.numPages;

			pages = [];
			for (let i = 1; i <= noOfPages; i++) {
				pages.push(`page_#${i}`);
			}

			tick().then(() => {
				startRendering()
			});
		});
	}

	const startRendering = async () => {
		for (let i = 1; i <= canvasArray.length; i++) {
			let canvas = canvasArray[i-1];
			let page = await pdfDocument.getPage(i);

			let viewport = page.getViewport({ scale }) as PageViewport;
			let canvasContext = canvas.getContext("2d");

			canvas.height = viewport.height;
			canvas.width = viewport.width;

			let renderContext = { canvasContext, viewport } as RenderParameters;
			page.render(renderContext); // returns renderTask
		}
	}

	const changeScale = (increase: boolean) => {
        if (increase) {
            scale += scaleIncrement;
			startRendering();
        } else if (scale != 1) {
            scale -= scaleIncrement;
			startRendering();
        }

    }

	const printPDF = async () => {
        let base64PDF = await blobToBase64(src as unknown as Blob);
        const indexOfComma = base64PDF.indexOf(",");
        if (indexOfComma != -1) {
            base64PDF = base64PDF.substring(indexOfComma + 1);
        }
        
        PrintJS({
			printable: base64PDF,
			type: "pdf",
			base64: true,
			onLoadingEnd: onPrinterLoadingEnd,
			showModal: true,
			modalMessage: `Loading document ${name}`
		});
    }

	const onPrinterLoadingEnd = () => {
		log("Printer loading ends\n");
		onPrintEnd();
	}

	onMount(startIfReady);
</script>

<div class="h-full w-full">
	<div class="h-full w-full grid grid-flow-row">
		<div class="flex flex-row w-full h-10 p-1 border-b-[1px]">
			<div class="flex justify-start items-center text-sm">{name}</div>
			<div id="controls" class="flex justify-end items-center w-full pr-9">
				<div class="flex flex-row gap-3 items-center">
                    <Separator orientation="vertical" class="h-7"/>
					<button tabindex={-1} onclick={() => changeScale(true)}>
						<ZoomIn class="h-5 w-5 cursor-pointer hover:text-primary" />
					</button>
                    <div>
						<Input class="w-14 h-7 px-1 text-center" readonly tabindex={-1} value={scalePercent}/>
					</div>
					<button tabindex={-1} onclick={() => changeScale(false)}>
						<ZoomOut class="h-5 w-5 cursor-pointer hover:text-primary" />
					</button>
                    <Separator orientation="vertical" class="h-7"/>
					<button tabindex={-1} onclick={printPDF}>
						<Printer class="h-5 w-5 cursor-pointer hover:text-primary"/>
					</button>
					<!-- <div><Download class="h-5 w-5 cursor-pointer hover:text-primary"/></div> -->
				</div>
			</div>
		</div>
		<ScrollArea orientation="both" class="h-[94vh]">
			{#each pages as page, idx}
				<div class="mt-2 relative flex justify-center" data-page={page}>
					<canvas bind:this={canvasArray[idx]}></canvas>
				</div>
			{/each}
		</ScrollArea>
	</div>
</div>

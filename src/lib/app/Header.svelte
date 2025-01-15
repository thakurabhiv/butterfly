<script>
    import * as Menubar from '$lib/components/ui/menubar';
    import { Separator } from "$lib/components/ui/separator";

    import { setMode } from 'mode-watcher';

    export let logInOK = false;
    /**
	 * @type {undefined}
	 */
     export let title = undefined;
</script>

<div class="flex items-center">
    <div class="inline-flex items-center mr-auto">
        <Menubar.Root class="border-none bg-background">
            <Menubar.Menu>
                <Menubar.Trigger>File</Menubar.Trigger>
                <Menubar.Content>
                    <Menubar.Item>Exit</Menubar.Item>
                </Menubar.Content>
            </Menubar.Menu>
        </Menubar.Root>
    </div>
    <div class="flex items-center">
        <div class="font-extralight text-lg">{ title ? title : '' }</div>
    </div>
    <div class="flex items-center mr-[5px] ml-auto">
        <Menubar.Root class="border-none">
            <Menubar.Menu>
                <Menubar.Trigger class="cursor-pointer hover:bg-secondary">user_001</Menubar.Trigger>
                <Menubar.Content>
                    <Menubar.Sub>
                        <Menubar.SubTrigger>Theme</Menubar.SubTrigger>
                        <Menubar.SubContent>
                            <Menubar.Item on:click={() => setMode('light')}>Light</Menubar.Item>
                            <Menubar.Item on:click={() => setMode('dark')}>Dark</Menubar.Item>
                        </Menubar.SubContent>
                    </Menubar.Sub>
                    <Separator/>
                    <Menubar.Item on:click={() => logInOK = false}>Logout</Menubar.Item>
                    <Menubar.Item>Exit</Menubar.Item>
                </Menubar.Content>
            </Menubar.Menu>
        </Menubar.Root>
    </div>
</div>
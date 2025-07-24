<script lang="ts">
	import '../app.css';
	import { WindowControls } from "$lib/components/window-controls";
	import { NoContextMenu } from '$lib/components/no-context-menu';
	import { SeamlessStartup } from '$lib/components/seamless-startup';
	import { isTauri } from "@tauri-apps/api/core";
	let { children } = $props();
</script>

{#if isTauri()}
	<!-- Comment this if you dont want to disable context menu -->
	<NoContextMenu/>

	<!-- Comment this if you dont want seamless startup -->
	<!-- Also change the "visible" windows configuration in tauri.conf.json if you comment this -->
	<SeamlessStartup/>

	<div aria-label="Drag region" class="fixed top-0 w-full flex justify-end" data-tauri-drag-region>
		<WindowControls/>
	</div>
{/if}

{@render children()}

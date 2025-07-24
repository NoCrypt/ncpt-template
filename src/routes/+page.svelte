<script lang="ts">
	import { Input } from "$lib/components/ui/input";
	import { Button } from "$lib/components/ui/button";
	import { invoke } from "@tauri-apps/api/core";
    import { fly } from "svelte/transition";
	import { isTauri } from "@tauri-apps/api/core";

	let name = $state("");
	let greetMsg = $state("");
	async function greet(event: Event) {
		greetMsg = "";
		greetMsg = isTauri() ? await invoke("greet", { name }) : await Promise.resolve(`Hello ${name}, this is a non-tauri environment!`);
	}
</script>


<main class="w-full h-full flex flex-col justify-center items-center gap-5 bg-linear-to-bl from-primary/35 to-background">
	<h1 class="font-bold text-center text-3xl mb-5">NoCrypt's Tauri Template</h1>
	<form class="flex sm:flex-row flex-col gap-5" onsubmit={greet}>
		<Input placeholder="Enter your name..." autofocus class="bg-transparent w-min" bind:value={name}/>
		<Button variant="default" type="submit">Send</Button>
	</form>
	<div class="relative mx-auto text-center w-full">
		{#key greetMsg}
		<p class="text-zinc-500 absolute mx-auto text-center w-full" in:fly={{y:20, duration:200}} out:fly={{y:-20, duration:200}}>{greetMsg}</p>
		{/key}
	</div>
</main>


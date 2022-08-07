<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import TwoColumnLayout from '../layouts/TwoColumnLayout.svelte';
	import init, { parse_markdown_to_json_blocks } from '../lib/fretta-client/pkg/fretta_client';
	let wasmInit = false;
	let lastUpdated = new Date();
	//TODO add type
	let jsonOutput: any[] = [];

	const handleTextInput = async (e: Event) => {
		if (!wasmInit) {
			await init();
		}
		if (e.target) {
			const value = (e.target as HTMLTextAreaElement).value;
			console.log(value);
			const jsonBlocks = parse_markdown_to_json_blocks(value);
			jsonOutput = jsonBlocks;
			lastUpdated = new Date();
		}
	};
</script>

<svelte:head>
	<title>Compose Page</title>
	<meta name="description" content="Compose Page" />
</svelte:head>

<section data-updated={lastUpdated.toString()}>
	<TwoColumnLayout leftPercentage={50}>
		<div class="edit" slot="left">
			<h2>Edit</h2>
			<textarea on:input={async (e) => await handleTextInput(e)} />
		</div>
		<div class="preview" slot="right">
			<h2>Output</h2>
			<div id="output">
				{#each jsonOutput as block}
					<div class="block">
						<div class="type">{block.block_type} - {block.start_line} : {block.end_line}</div>
						<div class="content">{block.text}</div>
					</div>
				{/each}
			</div>
		</div>
	</TwoColumnLayout>
</section>

<style lang="scss">
	section {
		margin-block: 2rem;
		.edit {
			textarea {
				width: 100%;
				min-height: 15rem;
			}
		}
		.preview {
			.content {
				white-space: pre-wrap;
			}
		}
	}
</style>

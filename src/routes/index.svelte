<script context="module" lang="ts">
	export const prerender = true;

	const getDisplayOffset = (lines: number, last: boolean, blockType: string) => {
		if(blockType.startsWith("h")){
			return 1;
		}
		if(blockType.startsWith("p")){
			console.debug(lines, last, blockType);
			return 1 + lines + (last ? 0 : 1);
		}
	}

</script>

<script lang="ts">
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
	<div class="edit">
		<h2>Edit</h2>
		<div class="article-wrapper">
			<div class="block-list">
				<ul>
					{#each jsonOutput as block, idx}
						<li data-start={block.start_line} data-end={block.end_line} style="--num-lines: {getDisplayOffset(block.end_line + 1 - block.start_line, idx == jsonOutput.length - 1, block.block_type)};">
							<div>{block.block_type}</div>
						</li>
					{/each}
				</ul>
			</div>
			<textarea class="article" on:input={async (e) => await handleTextInput(e)} />
		</div>
	</div>
</section>

<style lang="scss">
	section {
		margin-block: 2rem;
		.edit {
			.article-wrapper {
				display: flex;
				.block-list{
					flex: 0 0 var(--size-5);
					background-color: var(--background);
					ul{
						list-style-type: none;
						font-size: var(--font-size-2);
						margin: 0;
						padding-left: 0;
						li{
							line-height: calc(var(--num-lines) * var(--font-lineheight-0));
							border-top: 1px solid var(--text-color);
							border-bottom: 1px solid var(--text-color);
						}
					}
				}
				.article {
					flex: 1 0 auto;
					margin-right: var(--size-5);
					width: calc(100% - (2 * var(--size-5)));
					min-height: 15rem;
					line-height: var(--font-lineheight-0);
					border-radius: var(--radius-1);
					box-sizing: border-box;
					font-size: var(--font-size-2);
				}
			}
		}
		.preview {
			.content {
				white-space: pre-wrap;
			}
		}
	}
</style>

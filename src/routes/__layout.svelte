<script context="module" lang="ts">
	export const prerender = true;
</script>

<header>

</header>

<main>
	<slot />
</main>

<footer>
	<div class="footer-content">

	</div>
	<div class="doormat">
		Doormat
	</div>
</footer>

<style lang="scss">
	:root{
		--background: var(--lime-4);
		--text-color: var(--gray-9);
	}
	:global(body){
		font-size: 62.5%;
		font-family: 'Roboto', sans-serif;
		color: var(--text-color);
		margin: 0;
	}

	header{
		background-color: var(--background);
		height: 4rem;
		display: flex;
	}

	footer{
		background-color: var(--background);
		height: 8rem;
		display: flex;
		flex-direction: column;
		.footer-content{
			display: flex;
			flex: 1 0 6rem;
		}
		.doormat{
			width: 100%;
			flex: 0 0 2rem;
			background-color: var(--lime-2);
		}
	}

	main{
		margin-inline: 1rem;
		font-size: 1.5rem;
	}
</style>

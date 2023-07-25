<script lang="ts">
	let export_node: HTMLDivElement;

	function handle_export() {
		// make sure to save user progress before export.

		console.log('handle_export()');
	}
</script>

<div bind:this={export_node}>
	<h1>Export</h1>

	<button>Heart</button> Move dir: ./love <br />
	<button>Star3</button> Move dir: ./star3 <br />
	<button>Star2</button> Move dir: ./star2 <br />
	<button>Star1</button> Move dir: ./star1 <br />
	<button>Hate</button> Move dir: ./hate <br />

	<button on:click={handle_export}> Export </button>
</div>

<style>
</style>

<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { photo_map } from './stores';

	let export_node: HTMLDivElement;

	let export_settings = {
		heart: true,
		star3: true,
		star2: false,
		star1: false,
		hate: false,
		delete_original: false,
	}


	function handle_export() {
		// make sure to save user progress before export.

		const export_obj = Object.fromEntries($photo_map);


	}
</script>

<div bind:this={export_node}>
	<h1>Export</h1>

	<button>Heart</button> Copy dir: ./love <br />
	<button>Star3</button> Copy dir: ./star3 <br />
	<button>Star2</button> Copy dir: ./star2 <br />
	<button>Star1</button> Copy dir: ./star1 <br />
	<button>Hate</button> Copy dir: ./hate <br />

	<button>Delete original?</button> <span class="warning">Originals will be deleted.</span> <br />

	<button on:click={handle_export} class="export"> Export </button>
</div>

<style>
	button {
		margin: 0.5rem;
		height: 30px;
		border: 1px solid #aaa;
	}


	button.export{
		width: 100px;
		/* background-color: rgba(0, 191, 255, 0.2); */
		border: 1px solid dodgerblue;

	}


	div {
		color: #ccc;

	}

	.warning {
		color: rgb(255, 89, 0);
	}
</style>

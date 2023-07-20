<script lang="ts">
	import Heart from '$lib/Heart.svelte';
	import Star from './Star.svelte';
	import Xcross from './Xcross.svelte';
	import Center from './Center.svelte';
	import { current_photo, status } from '$lib/stores';

	export let choose_dir: () => void;
	//   export let filter: () => void;
	//   export let export_files: () => void;

	//   export let heart: () => void;
	//   export let star_1: () => void;
	//   export let star_2: () => void;
	//   export let star_3: () => void;

	// export let filename: string = 'untitled.jpg';

	export let center: () => void;
	export let settings: () => void;

	function heart(event) {
		console.log('heart');
		console.log(event);
	}
</script>

<div id="toolbar">
	<div class="group flex-start">
		<button id="choose-dir" on:click={choose_dir}>Choose folder</button>
		<button id="filter">Filter</button>
		<button id="export">Export</button>
		<div class="spacer" />
		<p>{$current_photo.photo_name}</p>
	</div>

	<div class="group center">
		<button>
			<Xcross fill_color="none" />
		</button>
		<div class="spacer" />
		<button>
			<Star fill_color="gray" />
		</button>
		<button>
			<Star fill_color="gray" />
		</button>
		<button>
			<Star fill_color="none" />
		</button>
		<div class="spacer" />
		<button on:click={heart}>
			<Heart />
		</button>
	</div>

	<div class="group flex-end">
		<p>{$status}</p>
		<div class="spacer" />

		<button on:click={center}>
			<Center fill_color="none" />
		</button>

		<!-- <button id="center" on:click={center}>Center</button> -->
		<button id="center">Goto</button>
		<button id="settings" on:click={settings}>Settings</button>
	</div>
</div>

<style lang="scss">
	div#toolbar {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
		height: 30px;
		width: 100vw;
		padding: 4px;
		background-color: rgba(37, 37, 37, 1);
	}

	button {
		height: 24;
		width: 24;

		background-color: rgba(0, 0, 0, 0);
		border: none;
		border-radius: 8px;
		align-items: center;
		justify-content: center;

		// max-width: 200px;
		// height: 28px;
		user-select: none;
		-webkit-user-select: none;
		-webkit-user-drag: none;

		transition: background-color 0.15s ease-in-out;

		color: rgb(150, 150, 150);
		// color: rgb(255, 130, 192);

		fill: none;
		
		&:hover {
			background-color: rgba(127, 127, 127, 0.2);
			// color: rgb(200, 200, 200);
			cursor: pointer;
			transition: background-color 0.15s ease-in-out;
		}

		&:active {
			background-color: rgba(127, 127, 127, 0.5);
			// color: rgb(222, 222, 222);
			// color: rgb(255, 130, 192);
			transition: background-color 0.1s ease-in-out;
		}
	}




	p {
		font-family: sans-serif;
		color: #aaa;
		user-select: none;
		-webkit-user-select: none;
		-webkit-user-drag: none;
		cursor: default;
	}

	.group {
		display: flex;
		flex-direction: row;
		flex-grow: 1;
		align-items: center;
		height: 30px;
		background-color: rgba(37, 37, 37, 1);
	}

	.flex-start {
		justify-content: flex-start;
	}

	.flex-end {
		justify-content: flex-end;
	}

	.center {
		justify-content: center;
	}

	.spacer {
		width: 2%;
		max-width: 10px;
	}
</style>

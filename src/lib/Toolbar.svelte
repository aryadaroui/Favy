<script lang="ts">
	import Heart from '$lib/Heart.svelte';
	import Star from './Star.svelte';
	import Xcross from './Xcross.svelte';
	import Center from './Center.svelte';
	import { current_photo, status } from '$lib/stores';
	import { onMount } from 'svelte';

	export let choose_dir: () => void;

	let loved: boolean = false;
	let hated: boolean = false;

	let star_1 = false;
	let star_2 = false;
	let star_3 = false;

	let rating = {
		val: 0,
		set(val: number) {
			this.val = val;
			star_1 = val >= 1;
			star_2 = val >= 2;
			star_3 = val >= 3;
		},
	};

	//   export let filter: () => void;
	//   export let export_files: () => void;

	//   export let heart: () => void;
	//   export let star_1: () => void;
	//   export let star_2: () => void;
	//   export let star_3: () => void;

	// export let filename: string = 'untitled.jpg';

	export let center: () => void;
	export let settings: () => void;

	function heart() {
		loved = !loved;
	}

	function xcross() {
		hated = !hated;
	}

	function star1() {
		if (star_1 == true) {
			rating.set(0);
		} else {
			rating.set(1);
		}
	}

	function star2() {
		if (star_2 == true) {
			rating.set(0);
		} else {
			rating.set(2);
		}
	}

	function star3() {
		if (star_3 == true) {
			rating.set(0);
		} else {
			rating.set(3);
		}
	}

	onMount(() => {
		// add keyboard shortcuts for ` 1 2 3 4
		document.addEventListener('keydown', (event) => {
			if (event.key == '`') {
				xcross();
			} else if (event.key == '1') {
				star1();
			} else if (event.key == '2') {
				star2();
			} else if (event.key == '3') {
				star3();
			} else if (event.key == '4') {
				heart();
			}
		});
	});
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
		<button on:click={xcross} class:xcross-selected={hated} class:xcross>
			<Xcross />
		</button>
		<div class="spacer" />
		<!-- BUG: Svelte can't find `class:star even though it's defined??` -->
		<button on:click={star1} class:star-selected={star_1 == true} class="star">
			<Star />
		</button>
		<button on:click={star2} class:star-selected={star_2 == true} class="star">
			<Star />
		</button>
		<button on:click={star3} class:star-selected={star_3 == true} class="star">
			<Star />
		</button>
		<div class="spacer" />
		<button on:click={heart} class:heart-selected={loved} class:heart>
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

		fill: rgba(0, 0, 0, 0.0);

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

	.heart {
		&:active {
			fill: rgba(255, 130, 192, 0.5);
			color: rgba(255, 130, 192, 0.5);
			transition: all 0.1s ease-in-out;
		}
	}
	.heart-selected {
		fill: rgb(255, 130, 192);
		color: rgb(255, 130, 192);
		transition: all 0.1s ease-in-out;
	}

	.xcross {
		&:active {
			fill: rgba(255, 120, 120, 0.5);
			color: rgba(255, 120, 120, 0.5);
			transition: all 0.1s ease-in-out;
		}
	}
	.xcross-selected {
		fill: rgb(255, 120, 120);
		color: rgb(255, 120, 120);
		transition: all 0.1s ease-in-out;
	}

	.star {
		&:active {
			fill: rgba(255, 230, 120, 0.5);
			color: rgba(255, 230, 120, 0.5);
			transition: all 0.1s ease-in-out;
		}
	}
	.star-selected {
		fill: rgb(255, 230, 120);
		color: rgb(255, 230, 120);
		transition: all 0.1s ease-in-out;
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

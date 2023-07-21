<script lang="ts">
	import Folder from '$lib/icons/Folder.svelte';
	import Filter from '$lib/icons/Filter.svelte';
	import BoxArrowUp from '$lib/icons/BoxArrowUp.svelte';

	import Xcross from '$lib/icons/Xcross.svelte';
	import Star from '$lib/icons/Star.svelte';
	import Heart from '$lib/icons/Heart.svelte';

	import Center from '$lib/icons/Center.svelte';
	import Crosshair from '$lib/icons/Crosshair.svelte';
	import Cog from '$lib/icons/Cog.svelte';

	import { current_photo, filter, status } from '$lib/stores';
	import { onMount } from 'svelte';

	export let choose_dir: () => void;

	let hear_selected: boolean = false;
	let xcross_selected: boolean = false;
	let star1_selected = false;
	let star2_selected = false;
	let star3_selected = false;

	let filter_selected = false;

	let filter_context: HTMLDivElement;

	let love = {
		val: 0,
		set(val: number) {
			this.val = val;
			if (val == 0) {
				hear_selected = false;
				xcross_selected = false;
			} else if (val == 1) {
				hear_selected = true;
				xcross_selected = false;
			} else if (val == -1) {
				hear_selected = false;
				xcross_selected = true;
			}
		},
	};

	let rating = {
		val: 0,
		set(val: number) {
			this.val = val;
			star1_selected = val >= 1;
			star2_selected = val >= 2;
			star3_selected = val >= 3;
		},
	};

	//   export let filter: () => void;
	//   export let export_files: () => void;

	//   export let heart: () => void;
	//   export let star_1: () => void;
	//   export let star_2: () => void;
	//   export let star_3: () => void;

	let xcross_down = false;
	let star1_down = false;
	let star2_down = false;
	let star3_down = false;
	let heart_down = false;

	export let center: () => void;
	export let settings: () => void;

	function filter_clicked() {
		// filter_selected = !filter_selected;
		filter_context.focus();
	}

	function heart_clicked() {
		love.set(love.val == 1 ? 0 : 1);
	}

	function xcross_clicked() {
		love.set(love.val == -1 ? 0 : -1);
	}

	function star1_clicked() {
		if (rating.val == 1) {
			rating.set(0);
		} else {
			rating.set(1);
		}
	}

	function star2_clicked() {
		if (rating.val == 2) {
			rating.set(0);
		} else {
			rating.set(2);
		}
	}

	function star3_clicked() {
		if (rating.val == 3) {
			rating.set(0);
		} else {
			rating.set(3);
		}
	}

	onMount(() => {
		// add keyboard shortcuts for ` 1 2 3 4
		document.addEventListener('keydown', (event) => {
			if (event.key == '`') {
				xcross_down = true;
			} else if (event.key == '1') {
				star1_down = true;
			} else if (event.key == '2') {
				star2_down = true;
			} else if (event.key == '3') {
				star3_down = true;
			} else if (event.key == '4') {
				heart_down = true;
			}
		});

		document.addEventListener('keyup', (event) => {
			if (event.key == '`') {
				xcross_clicked();
				xcross_down = false;
			} else if (event.key == '1') {
				star1_clicked();
				star1_down = false;
			} else if (event.key == '2') {
				star2_clicked();
				star2_down = false;
			} else if (event.key == '3') {
				star3_clicked();
				star3_down = false;
			} else if (event.key == '4') {
				heart_clicked();
				heart_down = false;
			}
		});
	});
</script>

<div id="toolbar">
	<div class="group flex-start">
		<button on:click={choose_dir}>
			<Folder />
		</button>

		
		<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
		<div bind:this={filter_context} class="filter-context" tabindex="0">
			<p>Filter stuff!</p>
		</div>
		<button class:filter-selected={filter_selected} on:click={filter_clicked}>
			<Filter />
		</button>

		<button>
			<BoxArrowUp />
		</button>
		<div class="spacer" />
		<p>{$current_photo.photo_name}</p>
	</div>

	<div class="group center">
		<button
			class="xcross"
			class:xcross-down={xcross_down}
			class:xcross-selected={xcross_selected}
			on:click={xcross_clicked}>
			<Xcross />
		</button>
		<div class="spacer" />
		<button
			class="star"
			class:star-down={star1_down}
			class:star-selected={star1_selected}
			on:click={star1_clicked}>
			<Star />
		</button>
		<button
			class="star"
			class:star-down={star2_down}
			class:star-selected={star2_selected}
			on:click={star2_clicked}>
			<Star />
		</button>
		<button
			class="star"
			class:star-down={star3_down}
			class:star-selected={star3_selected}
			on:click={star3_clicked}>
			<Star />
		</button>
		<div class="spacer" />
		<button
			class="heart"
			class:heart-down={heart_down}
			class:heart-selected={hear_selected}
			on:click={heart_clicked}>
			<Heart />
		</button>
	</div>

	<div class="group flex-end">
		<p>{$status}</p>
		<div class="spacer" />

		<button on:click={center}>
			<Center fill_color="none" />
		</button>

		<button>
			<Crosshair />
		</button>

		<button on:click={settings}>
			<Cog />
		</button>
	</div>
</div>

<style lang="scss">
	.filter-context {
		position: absolute;
		// top: -50px; /* adjust this value to position the div where you want it */
		left: 48px;
		bottom: 250px;
		border-radius: 8px;
		width: 200px;
		height: 100px;
		background-color: rgba(2, 2, 2, 0.2);
		// backdrop-filter: blur(9px);
		// border: 1px solid black;
		z-index: 1; /* make sure the div is above other elements */
		// visibility: hidden;

		&:focus {
			visibility: visible !important;
			border: 1px solid gray;
		}
	}


	div#toolbar {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
		// height: 30px;
		width: calc(100% - 8px);
		padding: 6px 4px;
		// padding-right: 20px;
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

		transition: all 0.15s ease-in-out;

		color: rgb(150, 150, 150);
		// color: rgb(255, 130, 192);

		fill: rgba(0, 0, 0, 0);

		&:hover {
			background-color: rgba(127, 127, 127, 0.2);
			// color: rgb(200, 200, 200);
			cursor: pointer;
			transition: background-color 0.1s ease-in-out;
		}

		&:active {
			background-color: rgba(127, 127, 127, 0.5);
			// color: rgb(222, 222, 222);
			// color: rgb(255, 130, 192);
			transition: background-color 0.1s ease-in-out;
			transform: scale(0.9);
		}
	}

	.filter-selected {
		background-color: rgb(220, 220, 220);
		color: rgb(32, 32, 32, 0);
		fill: rgb(32, 32, 32);
		transition: background-color 0.15s ease-in-out;

		&:hover {
			background-color: rgb(220, 220, 220, 0.8);
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
	.heart-down {
		fill: rgba(255, 130, 192, 0.5) !important;
		color: rgba(255, 130, 192, 0.5) !important;
		transform: scale(0.9);
		transition: all 0.1s ease-in-out;
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
	.xcross-down {
		fill: rgba(255, 120, 120, 0.5) !important;
		color: rgba(255, 120, 120, 0.5) !important;
		transform: scale(0.9);
		transition: all 0.1s ease-in-out;
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
	.star-down {
		fill: rgba(255, 230, 120, 0.5) !important;
		color: rgba(255, 230, 120, 0.5) !important;
		transform: scale(0.9);
		transition: all 0.1s ease-in-out;
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

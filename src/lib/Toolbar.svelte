<script lang="ts">
	import Folder from '$lib/icons/Folder.svelte';
	import Filter from '$lib/icons/Filter.svelte';
	import BoxArrowUp from '$lib/icons/BoxArrowUp.svelte';

	import Xcross from '$lib/icons/Xcross.svelte';
	import Star from '$lib/icons/Star.svelte';
	import Heart from '$lib/icons/Heart.svelte';

	import Photo from '$lib/icons/Photo.svelte';
	import Papers from '$lib/icons/Papers.svelte';

	import Center from '$lib/icons/Center.svelte';
	import Crosshair from '$lib/icons/Crosshair.svelte';
	import Cog from '$lib/icons/Cog.svelte';

	import { current_photo, filter, status } from '$lib/stores';
	import { onMount } from 'svelte';

	export let choose_dir: () => void;

	let filter_selected = false;
	let xcross_selected = false;
	let star1_selected = false;
	let star2_selected = false;
	let star3_selected = false;
	let heart_selected = false;

	let xcross_down = false;
	let star1_down = false;
	let star2_down = false;
	let star3_down = false;
	let heart_down = false;

	// let filter_menu_open = false;

	let filter_menu: HTMLDivElement;

	let love = {
		val: 0,
		set(val: number) {
			this.val = val;
			if (val == 0) {
				heart_selected = false;
				xcross_selected = false;
			} else if (val == 1) {
				heart_selected = true;
				xcross_selected = false;
			} else if (val == -1) {
				heart_selected = false;
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

	export let center: () => void;
	export let settings: () => void;

	function filter_clicked() {
		filter_selected = !filter_selected;
		// filter_menu_open = !filter_menu_open;
		// filter_menu.focus();
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
		// TODO: need flag for if any window or menu is focused, e.g. goto menu, settings menu, filter menu, etc.
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

		<button class="filter-button" class:filter-selected={filter_selected} on:click={filter_clicked}>
			<Filter />
		</button>
		<div class="filter-menu-hitbox hover-menu-hitbox">
			<div bind:this={filter_menu} class="hover-menu">
				<p>Filter stuff!</p>
			</div>
		</div>

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
			class:heart-selected={heart_selected}
			on:click={heart_clicked}>
			<Heart />
		</button>
	</div>

	<div class="group flex-end">

			{#if $status.text != ''}
				<p>{$status.text}</p>
			{:else}
			<span style="margin-right: 3px;"><Photo /> </span><p> {$status.reel.idx} / {$status.reel.len}</p> &nbsp; – &nbsp; <span><Papers /></span> <p>{$status.page.idx} / {$status.page.len}</p>
			{/if}

		<div class="spacer" />

		<button on:click={center}>
			<Center />
		</button>

		<button class="goto-button">
			<Crosshair />
		</button>
		<div class="goto-menu-hitbox hover-menu-hitbox">
			<div class="hover-menu">
				<p>Go to</p>

				photo #:<input type="text" />
				page #: <input type="text" />
				<hr />
				filename:<input type="text" />
			</div>
		</div>

		<button on:click={settings}>
			<Cog />
		</button>
	</div>
</div>

<style lang="scss">
	.hover-menu {
		border-radius: 16px;
		width: 200px;
		height: 200px;
		background-color: rgba(2, 2, 2, 0.5);
		z-index: 1; /* make sure the div is above other elements */
		overflow: hidden;
		-webkit-backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.25);
		box-shadow: 0px 8px 18px 0px rgba(0, 0, 0, 0.5);
		padding: 0 1em;
	}

	.hover-menu-hitbox {
		filter: blur(9px);
		visibility: hidden;
		opacity: 0;
		transition: all 0.15s ease-in-out;
		border-radius: 16px;
		padding: 2em 1em 1em 1em;

		// border: 1px solid pink;

		&:hover {
			visibility: visible;
			opacity: 1;
			filter: blur(0px);
			transform: translateY(-12px);
			transition: all 0.15s ease-in-out;
		}
	}

	.goto-button {
		&:hover + .hover-menu-hitbox {
			visibility: visible;
			opacity: 1;
			filter: blur(0px);
			transform: translateY(-12px);
			transition: all 0.15s ease-in-out;
		}
	}

	.goto-menu-hitbox {
		position: absolute;
		right: 30px;
		bottom: 220px;
	}

	.filter-button {
		&:hover + .filter-menu-hitbox {
			visibility: visible;
			opacity: 1;
			filter: blur(0px);
			transform: translateY(-12px);
			transition: all 0.15s ease-in-out;
		}
	}

	.filter-menu-hitbox {
		position: absolute;
		left: 30px;
		bottom: 220px;
	}

	.filter-selected {
		background-color: rgb(220, 220, 220);
		color: rgb(32, 32, 32, 0);
		fill: rgb(32, 32, 32);
		transition: background-color 0.15s ease-in-out;

		&:hover {
			background-color: rgb(220, 220, 220, 0.7);
			transition: background-color 0.1s ease-in-out;
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
		background-color: rgba(16, 16, 16, 1);

		border-top: 1px solid rgba(0, 0, 0, 0.5);
		border-bottom: 1px solid rgba(0, 0, 0, 0.5);
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

		color: #bbb;
		// color: rgb(255, 130, 192);

		fill: rgba(0, 0, 0, 0);

		&:hover {
			background-color: rgba(127, 127, 127, 0.2);
			// color: rgb(200, 200, 200);
			color: #eee;
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
		color: #777;
		user-select: none;
		-webkit-user-select: none;
		-webkit-user-drag: none;
		cursor: default;
	}

	span {
		color: #777;
	}

	.group {
		display: flex;
		flex-direction: row;
		flex-grow: 1;
		align-items: center;
		height: 30px;
		// background-color: rgba(37, 37, 37, 1);
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

	hr {
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	input {
		background-color: transparent;
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 0.2em;
		color: gray;
		width: 10em;
	}
</style>

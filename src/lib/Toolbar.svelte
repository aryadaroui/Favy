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

	import { current_photo, filter, status, photo_names, photo_map } from '$lib/stores';
	import { onMount } from 'svelte';

	export let choose_dir: () => void;
	export let on_export_clicked: () => void;

	let filter_selected = false;

	let rating_display = {
		star1: {
			selected: false,
			down: false,
		},
		star2: {
			selected: false,
			down: false,
		},
		star3: {
			selected: false,
			down: false,
		},
		set(val: number) {
			this.star1.selected = val >= 1;
			this.star2.selected = val >= 2;
			this.star3.selected = val >= 3;
			this.update();
		},
		update() {
			// sveltekit quirk: reassignment triggers reactive update
			rating_display = rating_display;
		},
	};

	let sentiment_display = {
		heart: {
			selected: false,
			down: false,
		},
		xcross: {
			selected: false,
			down: false,
		},
		set(val: number) {
			if (val == 0) {
				this.heart.selected = false;
				this.xcross.selected = false;
			} else if (val == 1) {
				this.heart.selected = true;
				this.xcross.selected = false;
			} else if (val == -1) {
				this.heart.selected = false;
				this.xcross.selected = true;
			}
			this.update();
		},
		update() {
			// sveltekit quirk: reassignment triggers reactive update
			sentiment_display = sentiment_display;
		},
	};

	$: $current_photo, update();

	function update() {
		// const photo_info = $photo_map.get($current_photo.photo_name);

		if ($photo_map.has($current_photo.photo_name)) {
			rating_display.set($photo_map.get($current_photo.photo_name)!.rating);
			sentiment_display.set($photo_map.get($current_photo.photo_name)!.love);
		} else {
			console.error('Toolbar.svelte: update(): photo_name not found in photo_map');
		}
	}

	let filter_menu: HTMLDivElement;

	export let center: () => void;
	export let settings: () => void;

	function filter_clicked() {
		filter_selected = !filter_selected;
	}

	function heart_clicked() {
		if (sentiment_display.heart.selected) {
			sentiment_display.set(0);
			$photo_map.get($current_photo.photo_name)!.love = 0;
		} else {
			sentiment_display.set(1);
			$photo_map.get($current_photo.photo_name)!.love = 1;
		}

		// sentiment_display.update();
	}

	function xcross_clicked() {
		if (sentiment_display.xcross.selected) {
			sentiment_display.set(0);
			$photo_map.get($current_photo.photo_name)!.love = 0;
		} else {
			sentiment_display.set(-1);
			$photo_map.get($current_photo.photo_name)!.love = -1;
		}
	}

	function star1_clicked() {
		if (rating_display.star1.selected) {
			rating_display.set(0);
			$photo_map.get($current_photo.photo_name)!.rating = 0;
		} else {
			rating_display.set(1);
			$photo_map.get($current_photo.photo_name)!.rating = 1;
		}
	}

	function star2_clicked() {
		if (rating_display.star2.selected) {
			rating_display.set(0);
			$photo_map.get($current_photo.photo_name)!.rating = 0;
		} else {
			rating_display.set(2);
			$photo_map.get($current_photo.photo_name)!.rating = 2;
		}
	}

	function star3_clicked() {
		if (rating_display.star3.selected) {
			rating_display.set(0);
			$photo_map.get($current_photo.photo_name)!.rating = 0;
		} else {
			rating_display.set(3);
			$photo_map.get($current_photo.photo_name)!.rating = 3;
		}
	}

	onMount(() => {
		// TODO: need flag for if any window or menu is focused, e.g. goto menu, settings menu, filter menu, etc.
		// TODO: have backspace remove everything
		// TODO: have ESC remove sentiment
		// TODO: make proper setters; the assign and .update() paattern is smelly.
		document.addEventListener('keydown', (event) => {
			if (event.key == '`') {
				sentiment_display.xcross.down = true;
				sentiment_display.update();
			} else if (event.key == '1') {
				// star1_down = true;
				rating_display.star1.down = true;
				rating_display.update();
			} else if (event.key == '2') {
				// star2_down = true;
				rating_display.star2.down = true;
				rating_display.update();
			} else if (event.key == '3') {
				// star3_down = true;
				rating_display.star3.down = true;
				rating_display.update();
			} else if (event.key == '4') {
				sentiment_display.heart.down = true;
				sentiment_display.update();
			}
		});

		document.addEventListener('keyup', (event) => {
			if (event.key == '`') {
				xcross_clicked();
				sentiment_display.xcross.down = false;
				sentiment_display.update();
			} else if (event.key == '1') {
				star1_clicked();
				rating_display.star1.down = false;
				rating_display.update();
			} else if (event.key == '2') {
				star2_clicked();
				rating_display.star2.down = false;
				rating_display.update();
			} else if (event.key == '3') {
				star3_clicked();
				rating_display.star3.down = false;
				rating_display.update();
			} else if (event.key == '4') {
				heart_clicked();
				sentiment_display.heart.down = false;
				sentiment_display.update();
			} else if (event.key == '0') {
				rating_display.set(0);
				rating_display.update();
			}
		});
	});
</script>

<div id="toolbar">
	<div class="group left">
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

		<button on:click={on_export_clicked}>
			<BoxArrowUp />
		</button>
		<div class="spacer" />
		<p>{$current_photo.photo_name}</p>
	</div>

	<div class="group center">
		<button
			class="xcross"
			class:xcross-down={sentiment_display.xcross.down}
			class:xcross-selected={sentiment_display.xcross.selected}
			on:click={xcross_clicked}>
			<Xcross />
		</button>
		<div class="spacer" />
		<button
			class="star"
			class:star-down={rating_display.star1.down}
			class:star-selected={rating_display.star1.selected}
			on:click={star1_clicked}>
			<Star />
		</button>
		<button
			class="star"
			class:star-down={rating_display.star2.down}
			class:star-selected={rating_display.star2.selected}
			on:click={star2_clicked}>
			<Star />
		</button>
		<button
			class="star"
			class:star-down={rating_display.star3.down}
			class:star-selected={rating_display.star3.selected}
			on:click={star3_clicked}>
			<Star />
		</button>
		<div class="spacer" />
		<button
			class="heart"
			class:heart-down={sentiment_display.heart.down}
			class:heart-selected={sentiment_display.heart.selected}
			on:click={heart_clicked}>
			<Heart />
		</button>
	</div>

	<div class="group right">
		{#if $status.text != ''}
			<p>{$status.text}</p>
		{:else}
			<span style="margin-right: 3px;"><Photo /> </span>
			<p>{$status.reel.idx} / {$status.reel.len} / {$photo_names.length}</p>
			<p>&nbsp; – &nbsp;</p>
			<span><Papers /></span>
			<p>{$status.page.idx} / {$status.page.len}</p>
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
				photo #:<input type="text" /> <br />
				page #: <input type="text" /> <br />
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

	// TODO: consolidate these syles into general classes.

	.hover-menu {
		border-radius: 16px;
		width: 200px;
		height: 200px;
		background-color: rgba(2, 2, 2, 0.8);
		z-index: 1; /* make sure the div is above other elements */
		overflow: hidden;
		-webkit-backdrop-filter: blur(12px);
		border: 1px solid rgba(255, 255, 255, 0.25);
		// border-top: 1px solid rgba(255, 255, 255, 0.5);
		// border-bottom: 1px solid rgba(255, 255, 255, 0.1);

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

		&:active {
			color: rgba(105, 222, 255, 0.5);
			fill: rgba(105, 222, 255, 0.5);
			transition: background-color 0.1s ease-in-out;
		}
	}

	.filter-menu-hitbox {
		position: absolute;
		left: 30px;
		bottom: 220px;
	}

	.filter-selected {
		color: rgb(105, 222, 255);
		fill: rgb(105, 222, 255);
		transition: background-color 0.15s ease-in-out;

		&:hover {
			color: rgb(105, 222, 255);
			fill: rgb(105, 222, 255);
			transition: background-color 0.1s ease-in-out;
		}
		&:active {
			color: rgba(105, 222, 255, 0.5);
			fill: rgba(105, 222, 255, 0.5);
			transition: background-color 0.1s ease-in-out;
		}
	}

	div#toolbar {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		// align-items: center;
		// height: 30px;
		// width: calc(100% - 8px);
		padding: 6px 6px;
		// padding-right: 20px;
		background-color: rgba(16, 16, 16, 1);

		border-top: 1px solid rgba(0, 0, 0, 0.5);
		border-bottom: 1px solid rgba(0, 0, 0, 0.5);
	}

	.heart {
		transition: all 0.1s ease-in-out;
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
		&:hover {
			color: rgb(255, 130, 192);
		}
	}

	.xcross {
		transition: all 0.1s ease-in-out;
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
		&:hover {
			color: rgb(255, 120, 120);
		}
	}

	.star {
		transition: all 0.1s ease-in-out;
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
		&:hover {
			color: rgb(255, 230, 120);
		}
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
		align-items: center;
		height: 30px;
		white-space: nowrap;
		// overflow: hidden;
	}

	.left {
		justify-content: flex-start;
		width: 500px;
		overflow: hidden;
	}

	.right {
		justify-content: flex-end;
		width: 500px;
		overflow: hidden;
	}

	.center {
		justify-content: center;
		width: 300px;
		z-index: 1;
	}

	.spacer {
		width: 1vw;
		max-width: 20px;
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
		height: 3em;
	}
</style>

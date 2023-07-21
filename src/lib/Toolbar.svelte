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

	import { current_photo, status } from '$lib/stores';
	import { onMount } from 'svelte';

	export let choose_dir: () => void;

	let loved: boolean = false;
	let hated: boolean = false;
	let starred_1 = false;
	let starred_2 = false;
	let starred3 = false;

	let love = {
		val: 0,
		set(val: number) {
			this.val = val;
			if (val == 0) {
				loved = false;
				hated = false;
			} else if (val == 1) {
				loved = true;
				hated = false;
			} else if (val == -1) {
				loved = false;
				hated = true;
			}
		},
	};

	let rating = {
		val: 0,
		set(val: number) {
			this.val = val;
			starred_1 = val >= 1;
			starred_2 = val >= 2;
			starred3 = val >= 3;
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
		love.set(love.val == 1 ? 0 : 1);
	}

	function xcross() {
		love.set(love.val == -1 ? 0 : -1);
	}

	function star1() {
		if (rating.val == 1) {
			rating.set(0);
		} else {
			rating.set(1);
		}
	}

	function star2() {
		if (rating.val == 2) {
			rating.set(0);
		} else {
			rating.set(2);
		}
	}

	function star3() {
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
		<button on:click={choose_dir}>
			<Folder />
		</button>
		<button id="filter">
			<Filter />
		</button>

		<button>
			<BoxArrowUp />
		</button>
		<div class="spacer" />
		<p>{$current_photo.photo_name}</p>
	</div>

	<div class="group center">
		<button on:click={xcross} class:xcross-selected={hated} class:xcross>
			<Xcross />
		</button>
		<div class="spacer" />
		<!-- BUG: Svelte can't find `class:star even though it's defined??` -->
		<button on:click={star1} class:star-selected={starred_1 == true} class="star">
			<Star />
		</button>
		<button on:click={star2} class:star-selected={starred_2 == true} class="star">
			<Star />
		</button>
		<button on:click={star3} class:star-selected={starred3 == true} class="star">
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

		<button>
			<Crosshair />
		</button>

		<button on:click={settings}>
			<Cog />
		</button>
	</div>
</div>

<style lang="scss">
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

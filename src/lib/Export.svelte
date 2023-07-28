<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { exists } from '@tauri-apps/api/fs';
	import { workspace_dir, photo_map } from './stores';

	import Heart from '$lib/icons/Heart.svelte';
	import Stars3 from '$lib/icons/Stars3.svelte';
	import Stars2 from '$lib/icons/Stars2.svelte';
	import Stars1 from '$lib/icons/Stars1.svelte';
	import Xcross from '$lib/icons/Xcross.svelte';
	import Trash from '$lib/icons/Trash.svelte';
	import ButtonToggle from '$lib/ButtonToggle.svelte';

	let export_node: HTMLDivElement;

	interface Dirs {
		existing: string[];
		check: () => Promise<void>;
		update: () => void;
	}

	$: $workspace_dir, check_dirs();

	export function check_dirs() {
		dirs.check();
	}

	let dirs: Dirs = {
		existing: [],
		async check() {
			let buf = [];
			if ($workspace_dir != '') {
				if (await exists($workspace_dir + 'love')) {
					buf.push('love');
				}
				if (await exists($workspace_dir + 'star3')) {
					buf.push('star3');
				}
				if (await exists($workspace_dir + 'star2')) {
					buf.push('star2');
				}
				if (await exists($workspace_dir + 'star1')) {
					buf.push('star1');
				}
				if (await exists($workspace_dir + 'hate')) {
					buf.push('hate');
				}
				this.existing = buf;
				console.log('existing: ', this.existing);
				this.update();
			}
		},
		update() {
			dirs = dirs;
		},
	};

	let settings = {
		favorite: false,
		star3: false,
		star2: false,
		star1: false,
		disliked: false,
		delete_original: false,
		update() {
			settings = settings;
		},
		set_heart(val: boolean) {
			settings.favorite = val;
			settings.update();
		},
		set_star3(val: boolean) {
			settings.star3 = val;
			settings.update();
		},
		set_star2(val: boolean) {
			settings.star2 = val;
			settings.update();
		},
		set_star1(val: boolean) {
			settings.star1 = val;
			settings.update();
		},
		set_disliked(val: boolean) {
			settings.disliked = val;
			settings.update();
		},
		set_delete_original(val: boolean) {
			settings.delete_original = val;
			settings.update();
		},
	};

	enum ExportStatus {
		Ready = 'ready',
		Working = 'working',
		Done = 'done',
		Failed = 'failed',
	}

	let export_status = ExportStatus.Ready;

	function handle_export() {
		// make sure to save user progress before export.

		export_status = ExportStatus.Working;
		const dir = $workspace_dir; // need to make a copy because $workspace_dir is a store; tauri doesn't like it.
		const photos = Array.from($photo_map, ([name, value]) => ({ name, ...value }));

		invoke('export', {
			dir,
			photos,
			settings,
		})
			.then((res) => {
				console.log(res);
				export_status = ExportStatus.Done;
			})
			.catch((err) => {
				console.log(err);
				export_status = ExportStatus.Failed;
			});
	}
</script>

<div bind:this={export_node} class="export_panel">
	<h1>Export</h1>
	<p>Selections to export</p>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_heart(is_on_after);
			}}>
			<Heart />
		</ButtonToggle>
		<span class="info-text" class:info-not-selected={!settings.favorite}><code> ./favorite</code></span>
		<br />
	</div>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_star3(is_on_after);
			}}>
			<Stars3 />
		</ButtonToggle>
		<span class="info-text" class:info-not-selected={!settings.star3}><code>./star3</code></span>
		<br />
	</div>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_star2(is_on_after);
			}}>
			<Stars2 />
		</ButtonToggle>
		<span class="info-text" class:info-not-selected={!settings.star2}><code>./star2</code></span>
		<br />
	</div>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_star1(is_on_after);
			}}>
			<Stars1 />
		</ButtonToggle>
		<span class="info-text" class:info-not-selected={!settings.star1}><code>./star1</code></span>
		<br />
	</div>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_disliked(is_on_after);
			}}>
			<Xcross />
		</ButtonToggle>
		<span class="info-text" class:info-not-selected={!settings.disliked}
			><code>./disliked</code></span>
		<br />
	</div>

	<div class="row">
		<ButtonToggle
			on_click={(is_on_after) => {
				settings.set_delete_original(is_on_after);
			}}>
			<Trash />
		</ButtonToggle>
		<span
			class:info-not-selected={!settings.delete_original}
			class:warning={settings.delete_original}
			>Original files
			{#if settings.delete_original}
				will
			{:else}
				will not
			{/if}
			be moved to trash</span> <br />
	</div>

	<div class="export-container">
		{#if $workspace_dir != ''}
			{#if dirs.existing.length != 0}
				<p class="warning">
					Target dirs: [./{dirs.existing.join(', ./')}] already exist. Existing files will be
					overwritten and directory will be polluted.
				</p>
			{/if}
		{:else}
			<p class="info-text">No workspace directory selected</p>
		{/if}
	</div>

	<div class="export-container">
		<button on:click={handle_export} class="export"> Export </button>
	</div>
</div>

<style lang="scss">
	code {
		font-size: 1.1em;
	}

	.export-container {
		display: flex;
		justify-content: center;
		align-items: center;
	}

	button.export {
		width: 100px;
		padding: 0 16px;
		height: 36px;
		font-size: medium;

		border: 1px solid dodgerblue;

		&:hover {
			background-color: #0b2d47;
			color: #ddd;
		}

		&:active {
			background-color: #065a9a;
		}
	}

	div {
		color: #ccc;
	}

	.info-text {
		margin-left: 8px;
		transition: color 0.2s ease-in-out;
	}

	.info-not-selected {
		color: #777;
		margin-left: 8px;
		transition: color 0.2s ease-in-out;
		// text-decoration: line-through; // might be too much
	}

	div.row {
		display: flex;
		align-items: center;
		margin: 0.4em;
	}

	span.slide-in {
		animation: forwards;

		@keyframes forwards {
			0% {
				opacity: 0;
				transform: translateX(-100%);
			}
			100% {
				opacity: 1;
				transform: translateX(0%);
			}
		}
	}

	p {
		margin: 1em 0.2em;
		user-select: none;
	}

	div.export_panel {
		position: relative;
		overflow-y: scroll;
		overflow-x: hidden;

		user-select: none;
		-webkit-user-select: none;
		-webkit-user-drag: none;
	}

	.warning {
		color: rgb(255, 191, 0);
		margin-left: 8px;
		transition: color 0.2s ease-in-out;
	}
</style>

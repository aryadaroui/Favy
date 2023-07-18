import { writable } from 'svelte/store';

export type PhotoName = string;
export type PhotoInfo = {
	hate: boolean,
	rating: number,
	love: boolean,
	thumbnail: Blob | null,
	idx: number,
};
export type Filter = {
	hate: {
		is_on: boolean,
		look_for: boolean,
	},
	rating: {
		is_on: boolean,
		look_for: number,
	},
	love: {
		is_on: boolean,
		look_for: boolean,
	},
};
export type Settings = {
	canvas: {
		transparency: number,
		visible: boolean,
	};
	toolbar: {
		transparency: number,
		visible: boolean,
	};
};

const initial_settings: Settings = {
	canvas: {
		transparency: 0.5,
		visible: true,
	},
	toolbar: {
		transparency: 0.5,
		visible: true,
	},
};
const initial_filter: Filter = {
	hate: {
		is_on: false,
		look_for: false,
	},
	rating: {
		is_on: false,
		look_for: 0,
	},
	love: {
		is_on: false,
		look_for: false,
	},
};


export const workspace_dir = writable('');
export const photo_names = writable([] as PhotoName[]);
export const photo_map = writable(new Map<PhotoName, PhotoInfo>());
export const current_photo = writable({ photo_name: '', idx: 0 });

export const filter = writable(initial_filter);
export const status = writable('ready');

export const settings = writable(initial_settings);

// export const cache
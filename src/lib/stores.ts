import { writable } from 'svelte/store';

type PhotoName = string;
type PhotoInfo = {
	hate: boolean,
	rating: number,
	love: boolean,
	thumbnail: Blob,
};

type Filter = {
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

type Settings = {
	canvas: {
		transparency: number,
		visible: boolean,
	};
	toolbar: {
		transparency: number,
		visible: boolean,
	};
};

export const workspace_dir = writable('');
export const photo_map = writable(new Map<PhotoName, PhotoInfo>());
export const current_photo = writable<PhotoName>('');

export const filter = writable({} as Filter);
export const status = writable('ready');

export const settings = writable({} as Settings);

// export const cache
import { json } from '@sveltejs/kit';
// import {fs} from 'node:fs';
import fs from 'fs';
import sharp from 'sharp';




export async function POST({ request }) {
	const { src_url, max_size } = await request.json();
	const img = await fs.readFileSync(src_url);


	const thumb_base64 = (await sharp(img).rotate().resize({ height: max_size }).jpeg().toBuffer()).toString('base64');

	return json({ thumb_base64 });
}
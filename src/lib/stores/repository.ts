import { writable } from 'svelte/store';
import type Repository from '$lib/types/repository.type';
import type { ReadRepo } from '$lib/types/commands.type';
import { invoke } from '@tauri-apps/api/tauri';

const repository = writable<Repository>({
	fullName: 'Seneca-CDOT/telescope',
	name: 'telescope',
	branch: 'master',
	localPath: '/Users/chrisbui/Documents/repos/telescope',
	commits: []
});

function openRepo(localPath: string) {
	return invoke<ReadRepo>('read_repo', {
		localRepo: localPath
	})
		.then(async ([repo, branch]) => {
			// console.log(repo, branch, commitLogs);
			// const commits = commitLogs.filter(Boolean).map((line) => {
			// 	const [sha, message] = line.split(/ (.*)/);
			// 	return {
			// 		sha,
			// 		message
			// 	};
			// });

			repository.set({
				name: repo,
				localPath,
				branch,
				commits: []
			});
		})
		.catch((error) => {
			console.error(error);
		});
}

function createStore() {
	return {
		subscribe: repository.subscribe,
		openRepo
	};
}

export default createStore();

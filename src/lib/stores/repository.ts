import { writable, get } from 'svelte/store';
import type { Writable } from 'svelte/store';
import type Repository from '$lib/types/repository.type';
import { invoke } from '@tauri-apps/api/tauri';
import type { ReadRepo, GetCommits } from '$lib/types/commands.type';
import type { Commit } from '$lib/types/repository.type';

type Store = {
	subscribe: Writable<Repository>['subscribe'];
	openRepo: (localPath: string) => Promise<void>;
	getCommits: (limit: number) => Promise<Commit[]>;
};

const createStore = (): Store => {
	const repository = writable<Repository>({
		fullName: 'Seneca-CDOT/telescope',
		name: 'telescope',
		branch: 'master',
		localPath: '/Users/chrisbui/Documents/repos/telescope',
		commits: []
	});

	async function openRepo(localPath: string) {
		try {
			const [repo, branch] = await invoke<ReadRepo>('read_repo', {
				localRepo: localPath
			});
			repository.set({
				name: repo,
				localPath,
				branch,
				commits: []
			});
		} catch (error) {
			console.error(error);
		}
	}

	async function getCommits(limit: number) {
		const commitLogs = await invoke<GetCommits>('get_commits', {
			localRepo: get(repository).localPath,
			limit
		});

		const commits = commitLogs.filter(Boolean).map((line) => {
			const [sha, message] = line.split(/ (.*)/);
			return {
				sha,
				message
			};
		});

		repository.update((repo) => ({ ...repo, commits }));
		return commits;
	}

	return {
		subscribe: repository.subscribe,
		openRepo,
		getCommits
	};
};

export default createStore();

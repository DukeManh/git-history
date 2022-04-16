import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import type { Writable } from 'svelte/store';
import type Repository from '$lib/types/repository.type';
import type { ReadRepo, GetCommits, GitShow, GitShowDiff } from '$lib/types/commands.type';
import type { Commit } from '$lib/types/repository.type';

const COMMITS_PER_LOAD = 150;

type Store = {
	subscribe: Writable<Repository>['subscribe'];
	openRepo: (localPath: string) => Promise<void>;
	getCommits: (before?: string) => Promise<Commit[]>;
	showCommit: (object: string) => Promise<GitShow | null>;
	showDiff: (object: string) => Promise<GitShowDiff | null>;
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

	async function getCommits(before = 'HEAD') {
		const commitLogs = await invoke<GetCommits>('get_commits', {
			localRepo: get(repository).localPath,
			limit: COMMITS_PER_LOAD,
			before
		});

		const commits = commitLogs.filter(Boolean).map((line) => {
			const [sha, message] = line.split(/ (.*)/);
			return {
				sha,
				message
			};
		});

		repository.update((repo) => ({ ...repo, commits: repo.commits.concat(commits) }));
		return commits;
	}

	async function showCommit(object: string) {
		try {
			const out = await invoke<GitShow>('git_show', {
				localRepo: get(repository).localPath,
				object
			});
			return out;
		} catch (error) {
			console.log(error);
		}
	}

	async function showDiff(object: string) {
		const out = await invoke<GitShowDiff>('git_show_diff', {
			localRepo: get(repository).localPath,
			object
		});
		return out;
	}

	return {
		subscribe: repository.subscribe,
		openRepo,
		getCommits,
		showCommit,
		showDiff
	};
};

export default createStore();

<div class="h-full overflow-scroll flex flex-col justify-start">
	<div class="w-full h-14 bg-slate-700  sticky top-0">
		<slot name="header" />
	</div>
	<div class="flex-1 min-h-0">
		<slot name="content" />
	</div>
</div>

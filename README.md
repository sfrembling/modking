# modking
A CLI universal mod swapper

# purpose
Have you ever downloaded a mod, gone to install it into your game, and then later wish to uninstall it?

Maybe you extracted some zip folder into the game's binary folder, overwriting whatever was there. 

Now that you don't want the mod anymore, the game's data is polluted with the mod. Your only choice is to re-install or recover manually.

This tool automates that process, all from the command line.

# what this does
This tool acts very similarly to [git](https://git-scm.com/), however the issue with using git for this purpose is that it will essentially duplicate the amount of space that your game's directory takes up on your disk.

This is because it keeps track of every single file and has a reference to them.

This tool differs by keeping a single "main" record of the directory's state, and then letting you install your mods as branches, only duplicating the necessary files.

For example, let's say you wanted to install [this mod](https://github.com/Lyall/P3RFix) for Persona 3 Reload. First, you'd go to P3R's directory and open the terminal, entering the following:

```shell
modk init
```

That will create a new folder `.modking` containing a single file, `repo.json`. 

Then run: 
```shell
modk add --all
```

This will hash every file in the game's directory and add them to the main record.

Finally, run:
```shell
modk lock
```

That will lock the current state of the vanilla game, meaning successive attempts to run `modk add` will be met with an error.
(You can use `modk unlock` to unlock this in the future)

The shorthand for the above steps is:
```shell
modk init --full
```

Now, to install the above mod, simply install as normal, then type the following:
```shell
modk swap --new P3RFix
modk add --all
```

Now, if you ever want to change which mod pack you're using, use `modk swap` and select your pack:

These "mod packs" are just changes that mod king keeps track of, so multiple mods can exist in one pack, assuming they're compatible.
Just remember to use `modk add` to add the files you need so you can swap back and forth.

# riva
A vim-like text editor written in Rust!

## Rationale
Text editors come and go, but they are and always shall be a necessity in the world of software development. While we are truly spoiled for choice for good IDEs and text editors, I wanted to try my hand at building a simple and extensible text editor that anyone can build atop of.

To that end, I have considered what exactly is "necessary" for a text editor. Of course, the ability to open files, edit, and save them is paramount. To this end, I believe there are many editor operations that can be pre-built into the editor to make writing anything much easier and less confusing. I believe that the Vim philosophy of keyboard-centrism is a very strong place to start. There are also plenty of Vim and Neovim plugins that add a lot to their existing capabilities. I would like to include those things.

However, it is very easy to get lost in a sea of features and builtins. Language parsers and debuggers aren't going to last forever and may be replaced by better software. File management, version control, and scripting languages are all ephemeral and changeable things. What we have now is great, but there are no guarantees that anything will be the "standard" for the next twenty, or even ten years. These will not be built into the editor. But I will provide a number of plugins that provide certain features so that anyone can immediately get started. That being said, all is replaceable.

Finally, while I am used to the way that Vim and Neovim organize commands in normal mode, I have always wondered by only one mode should exist. That is, certain editor functions may be better served grouped together under different modes. This may help with code organization, functionality, and a stronger understanding of what exactly is happening in the editor at any point in time. That being said, I wouldn't like to stray too hard from normal mode and take everything away from it. Baby steps. I'll move in this direction and see if it all makes sense. If not, then I don't mind letting it go either.

I am not and never will be some genius developer who can build a tool that does everything right. To the extent that Riva can edit text, I will do the best I can. To the extent that I can make it look nice and simple to configure, I will try. Hopefully others can take up my idea for what this editor should be and follow along and build something that is friendly, functional, and fun to use.

<3

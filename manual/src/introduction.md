# Introduction

Allay is a command-line application which can be used to create add-ons for Minecraft: Bedrock Edition.
It's goal is to take over the boring work you would do when manually developing an add-on such as generating
UUIDs and handling localization.

When working with Allay you work on the source files and Allay takes them and transforms them in the way you
need it so your add-on can be imported into Minecraft and published. This allows you to quickly build your
next big project.

Throughout this manual you will read the terms `pack` and `add-on` frequently which are often used
interchangeably and differently. Here, both `add-on` and `pack` refer to a "Behavior Pack", "Resource Pack",
"Skin Pack" or "World Template" which can be exported to Minecraft. Allay bundles these together to a single
"build" (which has the file extension `mcaddon` but may consist of multiple different add-ons).

# super-waddle
Example game using bevy game engine


## Step 1: A basic Bevy app with default plugins

Game programming requires you to write code that ultimately renders a frame to the player ~60x per second.

- `1000ms / 60 = 16.6666ms` per frame  (performance constraints similar to latency-sensitive web API programming)
- A common pattern for dealing with this is to keep most of what’s happening near the player available in the application’s memory; and to only execute logic on certain parts of the world when it’s needed.  `ECS` (Entity Component System) is a design pattern for accomplishing this.
- Bevy provides an application shell and programming tools to allow development of games using an ECS.
    - The `App` with `DefaultPlugins` contains a `World`, an event loop powered by a `Runner`, and a `Schedule` that defines in what part of the event loop will systems be run.


## Step 2: Load assets and render them to a 2d camera. 

### ECS basics
- `Entities` are objects instantiated by the programmer that exist in the `World`.
- `Components` are attached to entities, to hold data and/or indicate that entity should have some specific behavior.
- `Systems` are functions you write that *implements* the behaviors for each entity and component.


### Bevy implementation
- One primary type of parameter to a bevy system are `Queries` that allow you to select entities with specific components .
- Another is an object called `Commands` which allows you to manipulate entities in the world; spawn new ones, destroy existing ones, add or remove components.
- A third type are called `Resources`, which are like singleton entities with only one component; they are convenient for tracking global game state.
- A fourth type are called `EventReaders` and / or `EventWriters`; these allow for decoupling code across your application by using a pub / sub type of model.
- New code is registered to run via `Plugins`, which are a trait that you implement on a standard Rust struct. You need to implement a `fn build(&self, app: &mut App)` method, upon which you configure the `app` with whatever systems, events, resources, and other plugins you want to include.
- Systems can be ordered; by default they run in parallel when in the same part of the schedule (all systems scheduled to run in `Update` will run in parallel unless told otherwise)
    - use `.before(...)`, `.after(...)`, or `.chain(...)` when registering systems to order them
    - You can also use a `SystemSet` to declare system-ordering level dependencies across plugins, if necessary.
- You can use states to organize and coordinate game logic
    - Example: `Loading`, `Gaming`, `GameOver`

## Step 3: Read keyboard input, and move the player in response

Bevy uses its event system to send input from devices such as the keyboard, mouse, gamepad, etc...
In this step, we store the input in a resource so we can easily read the state of it from other systems. 
We also spawn some flowers so that we can see the movement happening. 


## Step 4: More advanced asset loading and game state organization

In this step, we get tired of waiting to recompile to change magic numbers like _player speed_ and _num flowers_. So, we implement a custom type of AssetLoader that uses `serde_json` to read a json file. 
In doing so, we learn about the importance of plugin initialization order (put the default plugins first, then any 3p libraries, then your own code).  
We use an advanced technique called a [`SystemParam`](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/system_param.rs), which allows us to implement a struct that we can use to easily read values out of the config file in other systems. 

Once there, we run the app only to find that now there's an ordering dependency between loading the config file, and the systems that initialize the player and flowers (symptom being that the flowers failed to spawn👎). 

To fix, we introduce the usage of [`bevy_asset_loader`](https://github.com/NiklasEi/bevy_asset_loader) and leverage its ability to hook into bevy's [`State`](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/state.rs) construct. 

As a bonus, bevy asset loader allows us to decouple the configuration of the assets, so that each plugin can declare its own assets that it depends upon without knowing about the others. 

## Step 5: Player & Enemy interaction, UI basics, & sustainable game architecture

In this step, we add some game to this game, by making it so the player picks up flowers when they get in range. We add a [text UI](https://github.com/bevyengine/bevy/blob/latest/examples/ui/text.rs) to display the number of flowers picked. 

This demonstrates the usage of Queries to select and iterate over entities, [events](https://docs.rs/bevy/latest/bevy/ecs/event/index.html) to decouple behavior and fanout notifications, and [change-tracking query filters](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Changed.html) to build a UI decoupled from the counting behavior. 

## Step 6: Add a countdown timer and end-of-game scoring

This adds a new plugin that advances states once the timer runs out. It also includes an example of handling interactions on a Bevy UI Button.


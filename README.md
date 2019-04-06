# The LV2 Book - Rust Edition

## Foreword

This is a translation of the [LV2 Book by David Robillard](http://lv2plug.in/book/) for the [`lv2rs`](https://github.com/Janonard/lv2rs.git) library. As such, the examples in this book have the same behaviour as in the original, but the book itself has been altered to adapt for the differences between C and Rust.

## Introduction

This is a series of well-documented example plugins that demonstrate the various features of LV2. Starting with the most basic plugin possible, each adds new functionality and explains the features used from a high level perspective.

API and vocabulary reference documentation explains details, but not the ``big picture''. This book is intended to complement the reference documentation by providing good reference implementations of plugins, while also conveying a higher-level understanding of LV2.

The chapters/plugins are arranged so that each builds incrementally on its predecessor. Reading this book front to back is a good way to become familiar with modern LV2 programming. The reader is expected to be familiar with Rust, but otherwise no special knowledge is required; the first plugin describes the basics in detail.

Each chapter corresponds to executable plugin code which can be found in the `samples` directory of the book's [Github Repository](https://github.com/Janonard/lv2rs-book). If you prefer to read actual source code, all the content here is also available in the source code as comments.

## Simple amplifier

This plugin is a simple example of a basic LV2 plugin with no additional features. It has audio ports which contain an array of float, and a control port which contains a single float.

LV2 plugins are defined in two parts: code and data. The code is written in any C compatible language, in our case Rust. Static data is described separately in the human and machine friendly Turtle syntax.

Generally, the goal is to keep code minimal, and describe as much as possible in the static data. There are several advantages to this approach:

* Hosts can discover and inspect plugins without loading or executing any plugin code.
* Plugin data can be used from a wide range of generic tools like scripting languages and command line utilities.
* The standard data model allows the use of existing vocabularies to describe plugins and related information.
* The language is extensible, so authors may describe any data without requiring changes to the LV2 specification.
* Labels and documentation are translatable, and available to hosts for display in user interfaces.

### Crate setup

The usual setup of a LV2 plugin created with `lv2rs` is based on a single library crate which you can create with the following cargo command:

    cargo new --lib eg-amp-rs

The `Cargo.toml` is pretty simple:

    [package]
    name = "eg-amp-rs"
    version = "0.2.0"
    authors = ["Janonard <janonard@protonmail.com>"]
    license = "ISC"
    edition = "2018"

    [lib]
    crate-type = ["dylib"]

    [dependencies]
    lv2rs = "0.3.0"

There is only one unusual thing: The crate type is set to "dylib". Usually, Rust library are statically linked objects which can be used to build other Rust libraries or executables. However, plugins need to be loaded at runtime by a host, which usually is written in C. Therefore, the compilation result of a plugin needs to be a dynamically linked library or shared object.

### `manifest.ttl`

LV2 plugins are installed in a `bundle`, a directory with a standard structure. Each bundle has a Turtle file named `manifest.ttl` which lists the contents of the bundle.

Hosts typically read the manifest of every installed bundle to discover plugins on start-up, so it should be as small as possible for performance reasons. Details that are only useful if the host chooses to load the plugin are stored in other files and linked to from `manifest.ttl`.

In a crate, this should be a special folder that contains the Turtle files. After the crate was build, the resulting shared object should also be copied into this folder.

#### URIs

LV2 makes use of URIs as globally-unique identifiers for resources. For example, the ID of the plugin described here is `<urn:lv2rs-book:eg-amp-rs>`. Note that URIs are only used as identifiers and don't necessarily imply that something can be accessed at that address on the web (though that may be the case).

#### Namespace Prefixes

Turtle files contain many URIs, but prefixes can be defined to improve readability. For example, with the `lv2:` prefix below, `lv2:Plugin` can be written instead of `<http://lv2plug.in/ns/lv2core#Plugin>`.

    @prefix lv2:  <http://lv2plug.in/ns/lv2core#> .
    @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

#### Describing a Plugin

Turtle files contain a set of statements which describe resources. This file contains 3 statements:

| Subject                      | Predicate      | Object       |
|------------------------------|----------------|--------------|
| `<urn:lv2rs-book:eg-amp-rs>` | `a `           | `lv2:Plugin` |
| `<urn:lv2rs-book:eg-amp-rs>` | `lv2:binary`   | `<amp.so>`   |
| `<urn:lv2rs-book:eg-amp-rs>` | `rdfs:seeAlso` | `<amp.ttl> ` |

Firstly, `<urn:lv2rs-book:eg-amp-rs>` is an LV2 plugin:

    <urn:lv2rs-book:eg-amp-rs> a lv2:Plugin .

The predicate `a` is a Turtle shorthand for `rdf:type`.

The binary of that plugin can be found at `<amp.ext>`:

    <urn:lv2rs-book:eg-amp-rs> lv2:binary <amp.so> .

This line is platform-dependet since it assumes that shared objects have the `.so` ending. For example on Windows, this line should look like this:

    <urn:lv2rs-book:eg-amp-rs> lv2:binary <amp.dll> .

Relative URIs in manifests are relative to the bundle directory, so this refers to a binary with the given name in the same directory as this manifest.

Finally, more information about this plugin can be found in `<amp.ttl>`:

    <urn:lv2rs-book:eg-amp-rs> rdfs:seeAlso <amp.ttl> .

### `amp.ttl`

The full description of the plugin is in this file, which is linked to from `manifest.ttl`.  This is done so the host only needs to scan the relatively small `manifest.ttl` files to quickly discover all plugins.

    @prefix doap:  <http://usefulinc.com/ns/doap#> .
    @prefix lv2:   <http://lv2plug.in/ns/lv2core#> .
    @prefix rdf:   <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
    @prefix rdfs:  <http://www.w3.org/2000/01/rdf-schema#> .
    @prefix units: <http://lv2plug.in/ns/extensions/units#> .

First the type of the plugin is described.  All plugins must explicitly list `lv2:Plugin` as a type.  A more specific type should also be given, where applicable, so hosts can present a nicer UI for loading plugins.  Note that this URI is the identifier of the plugin, so if it does not match the one in `manifest.ttl`, the host will not discover the plugin data at all.

    <urn:lv2rs-book:eg-amp-rs>
            a lv2:Plugin ,
                    lv2:AmplifierPlugin ;

Plugins are associated with a project, where common information like developers, home page, and so on are described.  This plugin is part of the v2rs-book project, which has URI <https://github.com/Janonard/lv2rs-book>, and is described elsewhere.  Typical plugin collections will describe the project in manifest.ttl

        lv2:project <https://github.com/Janonard/lv2rs-book> ;

Every plugin must have a name, described with the doap:name property. Translations to various languages can be added by putting a language tag after strings as shown later.

        doap:name "Simple Amplifier (Rust Version)" ,
        doap:license <http://opensource.org/licenses/isc> ;
        lv2:optionalFeature lv2:hardRTCapable ;
        lv2:port [

Every port must have at least two types, one that specifies direction (lv2:InputPort or lv2:OutputPort), and another to describe the data type. This port is a lv2:ControlPort, which means it contains a single float.

                a lv2:InputPort ,
                        lv2:ControlPort ;
                lv2:index 0 ;
                lv2:symbol "gain" ;
                lv2:name "Gain" ,
                        "收益"@ch ,
                        "Gewinn"@de ,
                        "Gain"@en-gb ,
                        "Aumento"@es ,
                        "Gain"@fr ,
                        "Guadagno"@it ,
                        "利益"@jp ,
                        "Увеличение"@ru ;

An lv2:ControlPort should always describe its default value, and usually a minimum and maximum value.  Defining a range is not strictly required, but should be done wherever possible to aid host support, particularly for UIs.

                lv2:default 0.0 ;
                lv2:minimum -90.0 ;
                lv2:maximum 24.0 ;

Ports can describe units and control detents to allow better UI generation and host automation.

                units:unit units:db ;
                lv2:scalePoint [
                        rdfs:label "+5" ;
                        rdf:value 5.0
                ] , [
                        rdfs:label "0" ;
                        rdf:value 0.0
                ] , [
                        rdfs:label "-5" ;
                        rdf:value -5.0
                ] , [
                        rdfs:label "-10" ;
                        rdf:value -10.0
                ]
        ] , [
                a lv2:AudioPort ,
                        lv2:InputPort ;
                lv2:index 1 ;
                lv2:symbol "in" ;
                lv2:name "In"
        ] , [
                a lv2:AudioPort ,
                        lv2:OutputPort ;
                lv2:index 2 ;
                lv2:symbol "out" ;
                lv2:name "Out"
        ] .

Include the lv2rs crate and the `CStr`

    extern crate lv2rs;

    use lv2rs::core::{self, *};
    use std::ffi::CStr;

Every plugin defines a private structure for the plugin instance.  All data associated with a plugin instance is stored here, and is available to every method.  In this simple plugin, only ports need to be stored, since there is no additional instance data.

    struct ExAmp {
        gain: ports::ParameterInputPort,
        input: ports::AudioInputPort,
        output: ports::AudioOutputPort,
    }

Everything a plugin needs to implement is the the `Plugin` trait. It contains all methods
required to make a plugin functional.

    impl Plugin for ExAmp {
    
The `instantiate` method is called by the host to create a new plugin instance. The host passes the plugin descriptor, sample rate, and bundle path for plugins that need to load additional resources (e.g. waveforms). The features parameter contains host-provided features defined in LV2 extensions, but this simple plugin does not use any.
    
This function is in the `instantiation` threading class, so no other methods on this instance will be called concurrently with it.

        fn instantiate(
            _descriptor: &Descriptor,
            _rate: f64,
            _bundle_path: &CStr,
            _features: Option<&FeaturesList>,
        ) -> Option<Self> {
            Some(Self {
                gain: ports::ParameterInputPort::new(),
                input: ports::AudioInputPort::new(),
                output: ports::AudioOutputPort::new(),
            })
        }

The `connect_port` method is called by the host to connect a particular port to a buffer.  The plugin must store the data location, but data may not be accessed except in run().
    
In code, ports are referred to by index and since neither nor other plugins can check if the pointers are actually valid for this type, you have to absolutely make sure that you map the right number to the right port. This is also the reason why it's unsafe, although nothing too unsafe does happen here.
    
This method is in the `audio` threading class, and is called in the same context as run().

        unsafe fn connect_port(&mut self, port: u32, data: *mut ()) {
            match port {
                0 => self.gain.connect(data as *const f32),
                1 => self.input.connect(data as *const f32),
                2 => self.output.connect(data as *mut f32),
                _ => (),
            }
        }

The `activate` method is called by the host to initialise and prepare the plugin instance for running.  The plugin must reset all internal state except for buffer locations set by `connect_port()`.  Since this plugin has no other internal state, this method does nothing. You do not even have to write it out if you don't need to, since it is already provided by the trait.
    
This method is in the `instantiation` threading class, so no other methods on this instance will be called concurrently with it.

        fn activate(&mut self) {}

The `run` method is the main process function of the plugin.  It processes  a block of audio in the audio context.  Since this plugin is `lv2:hardRTCapable`, `run()` must be real-time safe, so blocking (e.g. with a mutex) or memory allocation are not allowed.

        fn run(&mut self, n_samples: u32) {
            let input = unsafe { self.input.as_slice(n_samples) }.unwrap().iter();
            let output = unsafe { self.output.as_slice(n_samples) }
                .unwrap()
                .iter_mut();
            let gain = *(unsafe { self.gain.get() }.unwrap());

            // Convert the gain in dB to a coefficient.
            let coef = if gain > -90.0 {
                10.0f32.powf(gain * 0.05)
            } else {
                0.0
            };

            input
                .zip(output)
                .for_each(|(i_frame, o_frame)| *o_frame = *i_frame * coef);
        }

The `deactivate` method is the counterpart to `activate`, and is called by the host after running the plugin.  It indicates that the host will not call `run` again until another call to `activate` and is mainly useful for more advanced plugins with "live" characteristics such as those with auxiliary processing threads.  As with `activate`, this plugin has no use for this information so this method does nothing and it is provided by the trait.
    
This method is in the ``instantiation'' threading class, so no other methods on this instance will be called concurrently with it.
    
        fn deactivate(&mut self) {}

The `extension_data` function returns any extension data supported by the plugin. Note that this is not an instance method, but a function on the plugin descriptor.  It is usually used by plugins to implement additional interfaces. This plugin does not have any extension data, so this function returns None. Just like `activate` and `deactivate`, this function is already provided by the trait. 
    
This method is in the ``discovery'' threading class, so no other functions
or methods in this plugin library will be called concurrently with it.

        fn extension_data(_uri: &CStr) -> Option<&'static dyn ExtensionData> {
            None
        }
    }

If you know the original LV2 book, you might ask yourself "Where is the `cleanup` method?"
Well, there is none! Instead, plugins should implement the `Drop` trait to do cleaning. When
the host will call for cleanup, `lv2rs` will drop the plugin.

C programs, naturally, can not work with Rust structs implementing traits. Instead, hosts look
for one specific function called `lv2_descriptor` which returns all required pointers.

This function is generated by this macro. It takes the name of the `lv2rs_core` sub-crate in the
current namespace, the identifier of the plugin struct and the URI of the plugin.

The URI is the identifier for a plugin, and how the host associates this
implementation in code with its description in data. If this URI does not match that used
in the data files, the host will fail to load the plugin.

    lv2_main!(core, ExAmp, b"urn:lv2rs-book:eg-amp-rs\0");
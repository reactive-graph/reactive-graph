searchState.loadedDescShard("reactive_graph_plugin_api", 0, "Returns a collection of type ids which should be …\nReturns a collection of types which should be registered.\nReturns the id of the type provider.\nRegisters a factory for creating entity behaviours. If an …\nUnregisters a factory for creating entity behaviours.\nUnregisters the behaviour factories for the given entity …\nRegisters a factory for creating entity component …\nUnregisters an entity component behaviour factory.\nUnregisters all factories with the given entity component …\nRegisters a factory for creating relation behaviours. If a …\nUnregisters a factory for creating relation behaviours.\nUnregisters the behaviour factories for the given relation …\nRegisters a factory for creating relation component …\nUnregisters a factory for creating relation component …\nUnregisters the behaviour factories for the given …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nRuns the given GraphQL query.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nRegisters a web resource provider.\nUnregisters a web resource provider.\nThe context path segment.\nHandles a web resource.\nAdds the component with the given name to the entity …\nReturns the count of registered reactive entity instances.\nReturns the count of registered reactive entity instances …\nReturns the count of registered reactive entity instances …\nReturns the count of registered reactive entity instances …\nCreates a new reactive entity instance.\nDeletes the reactive entity instance with the given id.\nReturns the reactive entity instance with the given UUID …\nReturns all reactive entity instances.\nReturns the reactive entity instance with the given label …\nReturns the reactive entity instance and the matched path …\nReturns all reactive entity instances of the given type.\nReturns all ids.\nReturns true, if an entity instance exists with the given …\nRegisters a reactive entity instance and applies …\nRemoves the component with the given name from the entity …\nCreates a new reactive flow instance from the given flow …\nCreate a new reactive flow instance from the flow type by …\nDeletes the flow instance with the given id.\nReturns the flow instance with the given UUID or None.\nReturns the flow instance with the given label or None.\nReturns true, if an flow instance exists with the given …\nAdds the component with the given name to the relation …\nAdds the property with the given name and initial value to …\nReturns the count of registered reactive relation …\nReturns the count of registered reactive relation …\nReturns the count of registered reactive relation …\nReturns the count of registered reactive relation …\nCreates a new reactive relation instance.\nDeletes the reactive relation instance with the given key.\nReturns the argument unchanged.\nReturns the ReactiveRelation with the given type_name, …\nReturns all reactive relation instances.\nReturns all reactive relation instances of the given …\nReturns all reactive relation instances of the given …\nReturns all reactive relation instances of the given …\nReturns all reactive relation instances of the given type.\nReturns all relation instance ids.\nReturns true, if an relation of the given type exists …\nCalls <code>U::from(self)</code>.\nRegisters the given reactive relation instance and applies …\nRemoves the component with the given name from the …\nRemoves the property with the given name from the relation …\nCalled on initialization of the plugin.\nCalled on deactivation of the plugin.\nReturns the command manager.\nReturns the component import export manager.\nReturns the component manager.\nReturns the component provider registry.\nReturns the config manager.\nReturns the entity behaviour registry.\nReturns the entity component behaviour registry.\nReturns the entity instance manager.\nReturns the entity type import export manager.\nReturns the entity type manager.\nReturns the entity type provider registry.\nReturns the flow instance manager.\nReturns the flow type import export manager.\nReturns the flow type manager.\nReturns the flow type provider registry.\nReturns the GraphQL query service.\nReturns the relation behaviour registry.\nReturns the relation component behaviour registry.\nReturns the relation instance manager.\nReturns the relation type import export manager.\nReturns the relation type manager.\nReturns the relation type provider registry.\nReturns the system event manager.\nReturns the web resource manager.\nContains the registration\nReturns the plugin context.\nThe description of the plugin.\nReturns the argument unchanged.\nFunction to get the dependencies of the plugin.\nCalls <code>U::from(self)</code>.\nThe name of the plugin.\nThe version of plugin API. The version must match with the …\nThe library registrar function.\nRegisters the given plugin with the given name in the core …\nThe version of the rust compiler which has compiled the …\nThe version of the plugin.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nThe name of the dependency plugin.\nThe version of the dependency plugin.\nThe plugin is being activated.\nThe plugin failed to activate.\nThe plugin is running.\nThe plugin was compiled with an incompatible version of …\nThe plugin proxy is being constructed.\nThe plugin is being deactivated.\nAt least of of the dependencies are not active.\nThe plugin is being deployed.\nThe plugin has been disabled.\nThe runtime knows the plugin is there.\nThe plugin is installed.\nThe runtime has loaded the dynamic link library.\nThe plugin was compiled with an incompatible version of …\nThe plugin is compatible.\nThe runtime has loaded the plugin declaration.\nA plugin has one of these lifecycle states.\nThe plugin is being refreshed.\nThe providers of the plugin are being registered.\nThe plugin proxy is being destructed.\nThe plugin is there and all it’s prerequisites …\nThe state of the plugin is not yet complete\nThe plugin is being resolved.\nThe plugin is being started. If it has a init method, it …\nThe plugin is being started.\nThe plugin is being stopped. If it has a shutdown method, …\nThe plugin is being stopped.\nThe DLL is being deleted from file system.\nThe plugin has been removed from the runtime.\nThe plugin is being uninstalled.\nThe plugin is being uninstalled.\nThe DLL is being unloaded from memory.\nThe providers of the plugin are being unregistered.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nA thread-safe reference-counting pointer. ‘Arc’ stands …\nAdds the given entity instance.\nReturns a reference to the underlying allocator.\nProvides a raw pointer to the data.\nConverts to <code>Arc&lt;T&gt;</code>.\nConverts to <code>Arc&lt;[T]&gt;</code>.\nMakes a clone of the <code>Arc</code> pointer.\nComparison for two <code>Arc</code>s.\nDecrements the strong reference count on the <code>Arc&lt;T&gt;</code> …\nDecrements the strong reference count on the <code>Arc&lt;T&gt;</code> …\nCreates an empty str inside an Arc\nCreates an empty <code>[T]</code> inside an Arc\nCreates an empty CStr inside an Arc\nCreates a new <code>Arc&lt;T&gt;</code>, with the <code>Default</code> value for <code>T</code>.\nAttempts to downcast the <code>Arc&lt;dyn Any + Send + Sync&gt;</code> to a …\nDowncasts the <code>Arc&lt;dyn Any + Send + Sync&gt;</code> to a concrete …\nCreates a new <code>Weak</code> pointer to this allocation.\nDrops the <code>Arc</code>.\nEquality for two <code>Arc</code>s.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nAllocates a reference-counted <code>str</code> and copies <code>v</code> into it.\nConverts a <code>Path</code> into an <code>Arc</code> by copying the <code>Path</code> data into …\nConverts a <code>CString</code> into an Arc&lt;CStr&gt; by moving the <code>CString</code> …\nConverts a <code>&amp;CStr</code> into a <code>Arc&lt;CStr&gt;</code>, by copying the contents …\nConverts a <code>&amp;mut CStr</code> into a <code>Arc&lt;CStr&gt;</code>, by copying the …\nConverts a <code>T</code> into an <code>Arc&lt;T&gt;</code>\nAllocates a reference-counted slice and moves <code>v</code>’s items …\nReturns the argument unchanged.\nConverts an <code>OsString</code> into an Arc&lt;OsStr&gt; by moving the …\nCopies the string into a newly allocated Arc&lt;OsStr&gt;.\nCopies the string into a newly allocated Arc&lt;OsStr&gt;.\nMove a boxed object to a new, reference-counted allocation.\nAllocates a reference-counted <code>str</code> and copies <code>v</code> into it.\nAllocates a reference-counted <code>str</code> and copies <code>v</code> into it.\nConverts a <code>PathBuf</code> into an Arc&lt;Path&gt; by moving the <code>PathBuf</code> …\nConverts a <code>[T; N]</code> into an <code>Arc&lt;[T]&gt;</code>.\nAllocates a reference-counted slice and fills it by …\nConverts an atomically reference-counted string slice into …\nCreates an atomically reference-counted pointer from a …\nAllocates a reference-counted slice and fills it by …\nConverts a <code>Path</code> into an <code>Arc</code> by copying the <code>Path</code> data into …\nReturns the argument unchanged.\nTakes each element in the <code>Iterator</code> and collects it into an …\nConstructs an <code>Arc&lt;T&gt;</code> from a raw pointer.\nConstructs an <code>Arc&lt;T, A&gt;</code> from a raw pointer.\n‘Greater than or equal to’ comparison for two <code>Arc</code>s.\nReturns a mutable reference into the given <code>Arc</code>, if there …\nReturns a mutable reference into the given <code>Arc</code>, without …\nReturns a collection of type ids which should be …\nReturns a collection of type ids which should be …\nReturns a collection of types which should be registered.\nGreater-than comparison for two <code>Arc</code>s.\nReturns true, if an entity instance with the given id …\nReturns the id of the type provider.\nIncrements the strong reference count on the <code>Arc&lt;T&gt;</code> …\nIncrements the strong reference count on the <code>Arc&lt;T&gt;</code> …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts the reference-counted slice into a …\nReturns the inner value, if the <code>Arc</code> has exactly one strong …\nConsumes the <code>Arc</code>, returning the wrapped pointer.\nConsumes the <code>Arc</code>, returning the wrapped pointer and …\n‘Less than or equal to’ comparison for two <code>Arc</code>s.\nLess-than comparison for two <code>Arc</code>s.\nMakes a mutable reference into the given <code>Arc</code>.\nInequality for two <code>Arc</code>s.\nConstructs a new <code>Arc&lt;T&gt;</code>.\nConstructs a new <code>Arc&lt;T&gt;</code> while giving you a <code>Weak&lt;T&gt;</code> to the …\nConstructs a new <code>Arc&lt;T, A&gt;</code> in the given allocator while …\nConstructs a new <code>Arc&lt;T&gt;</code> in the provided allocator.\nConstructs a new <code>Arc</code> with uninitialized contents.\nConstructs a new <code>Arc</code> with uninitialized contents in the …\nConstructs a new atomically reference-counted slice with …\nConstructs a new atomically reference-counted slice with …\nConstructs a new <code>Arc</code> with uninitialized contents, with the …\nConstructs a new <code>Arc</code> with uninitialized contents, with the …\nConstructs a new atomically reference-counted slice with …\nConstructs a new atomically reference-counted slice with …\nPartial comparison for two <code>Arc</code>s.\nConstructs a new <code>Pin&lt;Arc&lt;T&gt;&gt;</code>. If <code>T</code> does not implement <code>Unpin</code>…\nConstructs a new <code>Pin&lt;Arc&lt;T, A&gt;&gt;</code> in the provided allocator. …\nReturns <code>true</code> if the two <code>Arc</code>s point to the same allocation …\nGets the number of strong (<code>Arc</code>) pointers to this …\nConstructs a new <code>Arc&lt;T&gt;</code>, returning an error if allocation …\nConstructs a new <code>Arc&lt;T, A&gt;</code> in the provided allocator, …\nConstructs a new <code>Arc</code> with uninitialized contents, …\nConstructs a new <code>Arc</code> with uninitialized contents, in the …\nConstructs a new <code>Arc</code> with uninitialized contents, with the …\nConstructs a new <code>Arc</code> with uninitialized contents, with the …\nConstructs a new <code>Pin&lt;Arc&lt;T&gt;&gt;</code>, return an error if …\nConstructs a new <code>Pin&lt;Arc&lt;T, A&gt;&gt;</code> in the provided allocator, …\nReturns the inner value, if the <code>Arc</code> has exactly one strong …\nIf we have the only reference to <code>T</code> then unwrap it. …\nGets the number of <code>Weak</code> pointers to this allocation.\nReturns a collection of types which should be registered.\nReturns the id of the type provider.\nReturns the command with the given name.\nReturns all commands.\nReturns the GraphQL server configuration.\nReturns the instance configuration.\nReturns the plugins configuration.\nReturns the remotes configuration.\nExports the component with the given type id to a JSON …\nImports a component from a JSON file located at the given …\nAdds an extension to the component with the given name.\nAdds a property to the component with the given name.\nReturns the count of components.\nReturns the count of components of the given namespace.\nCreates a new component with the given name and the given …\nDeletes the component with the given name.\nReturns all components whose names matches the given …\nReturns the component with the given name or empty.\nReturns all components\nReturns all components of the given namespace.\nReturns the component with the given fully qualified name …\nReturns true, if a component with the given name exists.\nReturns true, if a component with the given fully …\nRemoves the extension with the given type from the …\nRemoves the property with the given property_name from the …\nReplaces the component with the given name with the given …\nUpdates the description of the given component.\nReplaces the extension of the given component.\nAdds a property to the component with the given name.\nRegisters a component provider.\nUnregisters a component provider.\nExports the entity type with the given type id to a JSON …\nImports an entity type from a JSON file located at the …\nAdds the component with the given component_name to the …\nAdds an extension to the given entity type.\nAdds a property to the given entity type.\nReturns the count of entity types.\nReturns the count of entity types of the given namespace.\nCreates a new entity type.\nDeletes the entity type.\nReturns all entity types whose names matches the given …\nReturns the entity type with the given name or empty.\nReturns all entity types.\nReturns all entity types of the given namespace.\nReturns the entity type with the given fully qualified …\nReturns true, if a entity type with the given name exists.\nReturns true, if a entity type with the given fully …\nRemove the component with the given component_name from …\nRemoves the extension with the given type from the given …\nRemoves the property with the given property_name from the …\nUpdates the description of the given entity type.\nValidates the entity type with the given name. Tests that …\nRegisters an entity type provider.\nUnregisters an entity type provider.\nExports the flow type with the given type id to a JSON …\nImports a flow type from a JSON file located at the given …\nAdds the given entity instance to the flow type with the …\nAdds the given extension to the given flow type.\nAdds the given variable to the given flow type.\nReturns the count of flow types.\nReturns the count of flow types of the given namespace.\nCreates a new flow type.\nDeletes the flow type with the given name.\nReturns all flow types whose names matches the given …\nReturns the argument unchanged.\nReturns the flow type with the given name or empty.\nReturns all flow types.\nReturns all flow types.\nReturns the flow type with the given name or empty.\nReturns true, if a flow type with the given name exists.\nReturns true, if a flow type with the given name exists.\nCalls <code>U::from(self)</code>.\nRemoves the entity instance with the given id from the …\nRemoves the extension with the given type from the given …\nRemoves the variable with the given name from the flow …\nUpdates the description of the given flow type.\nUpdates the entity instance with the given id of the flow …\nUpdates the extension with the given type of the given …\nUpdates the variable with the given name of the flow type …\nValidates the flow type with the given name. Tests that …\nRegisters a flow type provider.\nUnregisters a flow type provider.\nRegisters a component provider.\nUnregisters a component provider.\nExports the relation type with the given type id to a JSON …\nImports a relation type from a JSON file located at the …\nAdds the component with the given type to the given …\nAdds an extension to the given relation type.\nAdds a property to the given relation type.\nReturns the count of relation types.\nReturns the count of relation types of the given namespace.\nCreates a new relation type.\nDeletes the given relation type.\nReturns all relation types whose names matches the given …\nReturns the argument unchanged.\nReturns the relation type with the given name.\nReturns all relation types.\nReturns all relation types of the given namespace.\nReturns the relation type with the given fully qualified …\nReturns true, if a relation type with the given name …\nReturns true, if a relation type with the given fully …\nCalls <code>U::from(self)</code>.\nRemove the component with the given type from the given …\nRemoves the extension with the given type from the given …\nRemoves the property with the given property_name from the …\nUpdates the description of the given relation type.\nUpdates the property with the given property_name. It’s …\nValidates the relation type with the given name. Tests …\nRegisters a relation type provider.\nUnregisters a relation type provider.\nReturns the reactive entity instance which can be …")
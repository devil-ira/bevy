window.SIDEBAR_ITEMS = {"enum":[["BatchResult",""],["RenderCommandResult",""]],"fn":[["batch_phase_system","This system batches the [`PhaseItem`]s of all [`RenderPhase`]s of this type."],["sort_phase_system","This system sorts all `RenderPhases` for the [`PhaseItem`] type."]],"struct":[["DrawFunctionId","An identifier for a [`Draw`] function stored in [`DrawFunctions`]."],["DrawFunctions","Stores all draw functions for the [`PhaseItem`] type hidden behind a reader-writer lock. To access them the [`DrawFunctions::read`] and [`DrawFunctions::write`] methods are used."],["DrawFunctionsInternal","Stores all draw functions for the [`PhaseItem`] type. For retrieval they are associated with their [`TypeId`]."],["RenderCommandState","Wraps a [`RenderCommand`] into a state so that it can be used as a [`Draw`] function. Therefore the [`RenderCommand::Param`] is queried from the ECS and passed to the command."],["RenderPhase","A resource to collect and sort draw requests for specific `PhaseItems`."],["SetItemPipeline",""],["TrackedRenderPass","A [`RenderPass`], which tracks the current pipeline state to ensure all draw calls are valid. It is used to set the current [`RenderPipeline`], `BindGroups` and buffers. After all requirements are specified, draw calls can be issued."]],"trait":[["AddRenderCommand","Registers a [`RenderCommand`] as a [`Draw`] function. They are stored inside the [`DrawFunctions`] resource of the app."],["BatchedPhaseItem","A [`PhaseItem`] that can be batched dynamically."],["CachedRenderPipelinePhaseItem",""],["Draw","A draw function which is used to draw a specific [`PhaseItem`]."],["PhaseItem","An item which will be drawn to the screen. A phase item should be queued up for rendering during the `RenderStage::Queue` stage. Afterwards it will be sorted and rendered automatically  in the `RenderStage::PhaseSort` stage and `RenderStage::Render` stage, respectively."],["RenderCommand","[`RenderCommand`] is a trait that runs an ECS query and produces one or more [`TrackedRenderPass`] calls. Types implementing this trait can be composed (as tuples)."]]};
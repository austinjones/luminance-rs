//! Tessellation gates.
//!
//! A tessellation gate is a _pipeline node_ that allows to share [`Tess`] for deeper nodes.
//!
//! [`Tess`]: crate::tess::Tess

use crate::backend::tess_gate::TessGate as TessGateBackend;
use crate::context::GraphicsContext;
use crate::tess::{TessIndex, TessVertexData, TessView};

/// Tessellation gates.
pub struct TessGate<'a, C>
where
  C: ?Sized,
{
  pub(crate) ctx: &'a mut C,
}

impl<'a, C> TessGate<'a, C>
where
  C: ?Sized + GraphicsContext,
{
  /// Enter the [`TessGate`] by sharing a [`TessView`].
  pub fn render<'b, T, V, I, W, S>(&'b mut self, tess_view: T)
  where
    C::Backend: TessGateBackend<V, I, W, S>,
    T: Into<TessView<'b, C::Backend, V, I, W, S>>,
    V: TessVertexData<S> + 'b,
    I: TessIndex + 'b,
    W: TessVertexData<S> + 'b,
    S: ?Sized + 'b,
  {
    let tess_view = tess_view.into();

    unsafe {
      self.ctx.backend().render(
        &tess_view.tess.repr,
        tess_view.start_index,
        tess_view.vert_nb,
        tess_view.inst_nb,
      )
    }
  }
}

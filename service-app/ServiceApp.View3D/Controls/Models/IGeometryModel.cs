using ServiceApp.View3D.Data.Models;
using ServiceApp.View3D.Render;

namespace ServiceApp.View3D.Controls.Models;

public interface IGeometryModel
{
    internal IDrawableObject GetOrCreateDrawable(VulkanContext context);
}
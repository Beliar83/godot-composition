#if TOOLS
using System.Diagnostics;
using System.Linq;
using System.Xml;
using GDExtension.Wrappers;
using Godot;

namespace GodotComposition;

[Tool]
public partial class GodotComposition : EditorPlugin
{
    private const string ItemGroupTagName = "ItemGroup";

    private const string ProjectReferenceTagName = "ProjectReference";

    private const string SourceGeneratorPath =
        "addons/godot_composition_cs/GodotCompositionSourceGenerator/GodotCompositionSourceGenerator.csproj";

    private const string IncludeElementName = "Include";
    private const string OutputItemTypeElementName = "OutputItemType";
    private const string ReferenceOutputAssemblyElementName = "ReferenceOutputAssembly";

    /// <inheritdoc />
    public override string _GetPluginName()
    {
        return "Godot Composition";
    }

    public override void _EnterTree()
    {
        Node godotCompositionEditorPlugin =
            GetParent().FindChildren("*", nameof(GodotCompositionEditorPlugin), owned: false).First();
        GodotCompositionEditorPlugin plugin = GodotCompositionEditorPlugin.Bind(godotCompositionEditorPlugin);

        RegisterComponentScripts(plugin);

        SetupGodotComposition();
        Process.Start("dotnet", ["build", SourceGeneratorPath]).WaitForExit();
    }

    static partial void RegisterComponentScripts(GodotCompositionEditorPlugin plugin);

    private void SetupGodotComposition()
    {
        string assemblyName = ProjectSettings.GetSetting("dotnet/project/assembly_name").ToString();
        XmlDocument document = new();
        document.Load(ProjectSettings.GlobalizePath($"res://{assemblyName}.csproj"));


        if (document.DocumentElement is null)
        {
            return;
        }

        XmlNodeList itemGroups = document.GetElementsByTagName(ItemGroupTagName);
        XmlElement? projectReferenceGroup = null;
        bool sourceGeneratorReferenceFound = false;

        foreach (XmlElement itemGroup in itemGroups)
        {
            if (sourceGeneratorReferenceFound)
            {
                break;
            }

            XmlNodeList projectReferences = itemGroup.GetElementsByTagName(ProjectReferenceTagName);
            if (projectReferences.Count == 0)
            {
                continue;
            }

            projectReferenceGroup = itemGroup;
            foreach (XmlElement projectReference in projectReferences)
            {
                string attribute = projectReference.GetAttribute(IncludeElementName);
                if (string.IsNullOrWhiteSpace(attribute))
                {
                    continue;
                }

                sourceGeneratorReferenceFound = attribute.SimplifyPath() == SourceGeneratorPath;
                if (sourceGeneratorReferenceFound)
                {
                    break;
                }
            }
        }

        if (sourceGeneratorReferenceFound)
        {
            return;
        }

        if (projectReferenceGroup is null)
        {
            projectReferenceGroup = document.CreateElement(ItemGroupTagName);
            document.DocumentElement.AppendChild(projectReferenceGroup);
        }

        XmlElement newProjectReference = document.CreateElement(ProjectReferenceTagName);
        newProjectReference.SetAttribute(IncludeElementName, SourceGeneratorPath);
        newProjectReference.SetAttribute(OutputItemTypeElementName, "Analyzer");
        newProjectReference.SetAttribute(ReferenceOutputAssemblyElementName, "false");
        projectReferenceGroup.AppendChild(newProjectReference);
        document.Save(ProjectSettings.GlobalizePath($"res://{assemblyName}.csproj"));
    }
}

#endif
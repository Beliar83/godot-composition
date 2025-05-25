using Microsoft.CodeAnalysis;

namespace GodotCompositionSourceGenerator;

// Based on code taken from the official Godot SourceGenerators at https://github.com/godotengine/godot/blob/master/modules/mono/editor/Godot.NET.Sdk/Godot.SourceGenerators/ExtensionMethods.cs
// Partially adjusted to work with incremental generators
public static class ExtensionMethods
{
    public static IncrementalValueProvider<(string? property, string? value)> SelectGlobalAnalyzerProperty(
        this IncrementalGeneratorInitializationContext context, string[] properties
    )
    {
        return context.AnalyzerConfigOptionsProvider.Select((provider, _) =>
        {
            foreach (string property in properties)
            {
                if (provider.GlobalOptions.TryGetValue("build_property." + property, out string? value) &&
                    !string.IsNullOrWhiteSpace(value))
                {
                    return (property, value);
                }
            }

            return ((string?)null, (string?)null);
        });
    }

    public static bool InheritsFrom(this ITypeSymbol? symbol, string assemblyName, string typeFullName)
    {
        while (symbol != null)
        {
            string fullQualifiedNameOmitGlobal = symbol.FullQualifiedNameOmitGlobal();
            if (symbol.ContainingAssembly?.Name == assemblyName &&
                fullQualifiedNameOmitGlobal == typeFullName)
            {
                return true;
            }

            symbol = symbol.BaseType;
        }

        return false;
    }

    private static SymbolDisplayFormat FullyQualifiedFormatOmitGlobal { get; } =
        SymbolDisplayFormat.FullyQualifiedFormat
            .WithGlobalNamespaceStyle(SymbolDisplayGlobalNamespaceStyle.Omitted)
            .WithGenericsOptions(SymbolDisplayGenericsOptions.None);

    private static string FullQualifiedNameOmitGlobal(this ITypeSymbol symbol)
    {
        return symbol.ToDisplayString(NullableFlowState.NotNull, FullyQualifiedFormatOmitGlobal);
    }

    public static bool IsGodotExportAttribute(this INamedTypeSymbol symbol)
    {
        return symbol.FullQualifiedNameOmitGlobal() == "Godot.ExportAttribute";
    }
}
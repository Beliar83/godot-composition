using System.Collections.Generic;
using System.Text;
using Godot;

namespace GDExtensionAPIGenerator;

internal static partial class CodeGenerator
{
    private static void ConstructSignals(
        ICollection<string> occupiedNames,
        IReadOnlyList<MethodInfo> signalList,
        StringBuilder codeBuilder,
        IReadOnlyDictionary<string, ClassInfo> inheritanceMap,
        IReadOnlyDictionary<string, string> godotSharpTypeNameMap,
        ICollection<string> builtinTypes,
        HashSet<string> nativeNameCache,
        string backingName
    )
    {
        if (signalList.Count != 0)
        {
            codeBuilder.AppendLine(
                """
                #region Signals

                """
            );
        }

        foreach (var signalInfo in signalList)
        {
            var returnValueName = signalInfo.ReturnValue.GetTypeName();

            var signalName = signalInfo.GetMethodName();

            if (occupiedNames.Contains(signalName))
            {
                signalName += "Signal";
            }
            else
            {
                occupiedNames.Add(signalName);
            }

            var signalDelegateName = $"{signalName}Handler";
            var signalNameCamelCase = ToCamelCase(signalName);
            var backingDelegateName = $"_{signalNameCamelCase}_backing";
            var backingCallableName = $"_{signalNameCamelCase}_backing_callable";

            codeBuilder.Append($"{TAB1}private void {signalNameCamelCase}Call(");
            BuildupMethodArguments(codeBuilder, signalInfo.Arguments, godotSharpTypeNameMap);
            codeBuilder.AppendLine(")");
            codeBuilder.AppendLine($"{TAB1}{{");

            const string argPrefix = "arg";

            static string Arg(int index) => $"{argPrefix}{index}";            
            
            for (var index = 0; index < signalInfo.Arguments.Length; index++)
            {
                var argumentInfo = signalInfo.Arguments[index];
                var convertedArgName = Arg(index);
                var argumentType = argumentInfo.GetTypeName();
                argumentType = godotSharpTypeNameMap.GetValueOrDefault(argumentType, argumentType);
                codeBuilder.Append($"{TAB2}var {convertedArgName} = ");
                string argumentName = argumentInfo.GetArgumentName();
                if (inheritanceMap.ContainsKey(argumentType))
                {
                    codeBuilder.AppendLine($"{STATIC_HELPER_CLASS}.{VariantToInstanceMethodName}<{argumentType}>({argumentName});");
                }
                else
                {
                    if (argumentInfo.IsArray)
                    {
                        var typeClass = godotSharpTypeNameMap.GetValueOrDefault(argumentInfo.TypeClass, argumentInfo.TypeClass);
                        if (inheritanceMap.ContainsKey(typeClass))
                            codeBuilder.AppendLine($"{STATIC_HELPER_CLASS}.{CastMethodName}<{typeClass}>({argumentName}));");
                        else
                            codeBuilder.AppendLine($"{argumentName});");
                    }
                    else
                        codeBuilder.AppendLine($"{argumentName};");
                }
            }

            var argumentsLength = signalInfo.Arguments.Length;
            codeBuilder.Append($"{TAB2}{backingDelegateName}?.Invoke(");

            for (var i = 0; i < argumentsLength; i++)
            {
                codeBuilder.Append(Arg(i));

                if (i != argumentsLength - 1)
                {
                    codeBuilder.Append(", ");
                }
            }

            codeBuilder.AppendLine(");");            
            codeBuilder.AppendLine($"{TAB1}}}");            
            
            codeBuilder.Append($"{TAB1}public delegate {returnValueName} {signalDelegateName}(");

            BuildupMethodArguments(codeBuilder, signalInfo.Arguments, godotSharpTypeNameMap);

            codeBuilder
                .AppendLine(");")
                .AppendLine();

            const string callableName = nameof(Callable);

            
            codeBuilder.Append(
                $$"""
                  {{TAB1}}private {{signalDelegateName}} {{backingDelegateName}};
                  {{TAB1}}private {{callableName}} {{backingCallableName}};
                  {{TAB1}}public event {{signalDelegateName}} {{signalName}}
                  {{TAB1}}{
                  {{TAB2}}add
                  {{TAB2}}{
                  {{TAB3}}if({{backingDelegateName}} == null)
                  {{TAB3}}{
                  {{TAB4}}{{backingCallableName}} = new {{callableName}}(this, MethodName.{{signalNameCamelCase}}Call);
                  
                  """
            );

            nativeNameCache.Add(signalInfo.NativeName);
            var signalCachedNativeName = NativeNameToCachedName(signalInfo.NativeName);
            
            codeBuilder.AppendLine(
                    $$"""
                      {{TAB4}}{{backingName}}Connect({{signalCachedNativeName}}, {{backingCallableName}});
                      {{TAB3}}}
                      {{TAB3}}{{backingDelegateName}} += value;
                      {{TAB2}}}
                      {{TAB2}}remove
                      {{TAB2}}{
                      {{TAB3}}{{backingDelegateName}} -= value;
                      {{TAB3}}
                      {{TAB3}}if({{backingDelegateName}} == null)
                      {{TAB3}}{
                      {{TAB4}}{{backingName}}Disconnect({{signalCachedNativeName}}, {{backingCallableName}});
                      {{TAB4}}{{backingCallableName}} = default;
                      {{TAB3}}}
                      {{TAB2}}}
                      {{TAB1}}}
                      """
                )
                .AppendLine();
        }

        if (signalList.Count != 0)
        {
            codeBuilder.AppendLine(
                """
                #endregion

                """
            );
        }
    }
}
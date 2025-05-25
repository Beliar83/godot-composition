using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices.JavaScript;
using System.Text;
using Godot;

namespace GDExtensionAPIGenerator;

internal static partial class CodeGenerator
{
    private static void ConstructMethods(HashSet<string> occupiedNames,
        IReadOnlyList<MethodInfo> methodInfoList,
        IReadOnlyDictionary<string, string> godotSharpTypeNameMap,
        IReadOnlyDictionary<string, ClassInfo> inheritanceMap,
        ICollection<string> builtinTypeNames,
        HashSet<string> nativeNameCache,
        StringBuilder stringBuilder,
        ClassInfo classInfo,
        string backing, string[] gdeTypeNames)
    {
        if (methodInfoList.Count == 0)
        {
            return;
        }
        stringBuilder.AppendLine(
            """
            #region Methods

            """
        );


        foreach (var methodInfo in methodInfoList)
        {
            nativeNameCache.Add(methodInfo.NativeName);
            var methodCachedNativeName = NativeNameToCachedName(methodInfo.NativeName);
            
            var returnValueName = methodInfo.ReturnValue.GetTypeName();

            var methodName = methodInfo.GetMethodName();

            if (occupiedNames.Contains(methodName))
            {
                methodName += "Method";
            }
            else
            {
                occupiedNames.Add(methodName);
            }

//              stringBuilder.AppendLine($"""
//                                        /*
//                                        {methodInfo}
//                                        */
//                                        """);

            stringBuilder
                .Append($"{TAB1}public ");

            var isVirtual = methodInfo.Flags.HasFlag(MethodFlags.Virtual);
            var isStatic = methodInfo.Flags.HasFlag(MethodFlags.Static);
            

            if (isStatic) stringBuilder.Append("static ");
            
            if (methodInfo.ReturnValue.IsArray)
            {
                returnValueName = returnValueName.Replace("Godot.GodotObject", godotSharpTypeNameMap.GetValueOrDefault(methodInfo.ReturnValue.TypeClass, methodInfo.ReturnValue.TypeClass));
            }
            
            stringBuilder
                .Append(returnValueName)
                .Append(' ')
                .Append(methodName)
                .Append('(');


            BuildupMethodArguments(stringBuilder, methodInfo.Arguments, godotSharpTypeNameMap);
            
            
            stringBuilder.Append(')');
            
            // TODO: VIRTUAL
            
            stringBuilder.Append(" => ");
            
            if (methodInfo.ReturnValue.IsArray && inheritanceMap.ContainsKey(methodInfo.ReturnValue.TypeClass))
            {
                stringBuilder.Append($"{STATIC_HELPER_CLASS}.{CastMethodName}<{methodInfo.ReturnValue.TypeClass}>(");
            }
            
            if (!methodInfo.ReturnValue.IsVoid &&
                inheritanceMap.TryGetValue(methodInfo.ReturnValue.ClassName, out var returnTypeInfo))
            {
                if (gdeTypeNames.Contains(returnTypeInfo.TypeName))
                {
                    stringBuilder.Append($"{returnTypeInfo.TypeName}.{VariantToInstanceMethodName}(");
                    
                }
                else
                {
                    stringBuilder.Append($"{STATIC_HELPER_CLASS}.{VariantToInstanceMethodName}<{returnTypeInfo.TypeName}>(");

                }
            }

            if (isStatic)
            {
                stringBuilder
                    .Append($"{STATIC_HELPER_CLASS}.")
                    .Append("Call(")
                    .Append(GDExtensionName)
                    .Append(", ")
                    .Append(methodCachedNativeName);
            }
            else
            {
                stringBuilder
                    .Append(backing)
                    .Append("Call(")
                    .Append(methodCachedNativeName);
            }

            if (methodInfo.Arguments.Length > 0)
            {
                stringBuilder.Append(", ");
                BuildupMethodCallArguments(
                    stringBuilder,
                    methodInfo.Arguments,
                    inheritanceMap,
                    godotSharpTypeNameMap,
                    builtinTypeNames
                );
            }
            
            // TODO: var isVararg = methodInfo.Flags.HasFlag(MethodFlags.Vararg);

            stringBuilder.Append(')');

            if (!methodInfo.ReturnValue.IsVoid)
            {
                if (inheritanceMap.TryGetValue(methodInfo.ReturnValue.ClassName, out returnTypeInfo))
                {
                    stringBuilder.Append($".{VariantToGodotObject})");
                }
                else
                {
                    if (!methodInfo.ReturnValue.IsArray || inheritanceMap.ContainsKey(methodInfo.ReturnValue.TypeClass))
                    {
                        stringBuilder.Append($".As<{methodInfo.ReturnValue.GetTypeName()}>()");
                    }
                    else
                    {
                        stringBuilder.Append($".As<{returnValueName}>()");
                    }
                }
            }
            
            if (methodInfo.ReturnValue.IsArray && inheritanceMap.ContainsKey(methodInfo.ReturnValue.TypeClass))
            {
                stringBuilder.Append(')');
            }
            
            stringBuilder.AppendLine(";").AppendLine();

            stringBuilder.AppendLine();

            // if (isVirtual)
            // {
            //     string virtualMethodName = $"_{methodInfo.NativeName.ToPascalCase()}";
            //
            //     stringBuilder
            //         .Append($"{TAB1}public virtual")
            //         .Append(' ')
            //         .Append(returnValueName)
            //         .Append(' ')
            //         .Append(virtualMethodName)
            //         .Append('(');
            //     BuildupMethodArguments(stringBuilder, methodInfo.Arguments, godotSharpTypeNameMap);
            //     stringBuilder.AppendLine(")");
            //     stringBuilder.AppendLine($"{TAB1}{{");
            //     if (!methodInfo.ReturnValue.IsVoid)
            //     {
            //         // This is suboptimal, but other than using a SourceGenerator to conditionally add the override
            //         // of the GDExtension function when the derived type overrides the generated virtual method
            //         // there is no other safe way to return a good default value
            //         stringBuilder.Append(
            //             $"{TAB2}return ClassDB.Instantiate(\"{classInfo.TypeName}\").AsGodotObject().Call(\"{methodInfo.NativeName}\").As<{returnValueName}>();");
            //         // stringBuilder.Append("return new Variant()");
            //         if (inheritanceMap.TryGetValue(methodInfo.ReturnValue.ClassName, out returnTypeInfo))
            //             // {
            //             //     stringBuilder.Append($".{VariantToGodotObject}");
            //             // }
            //             // else
            //             // {
            //             //     if (!methodInfo.ReturnValue.IsArray || inheritanceMap.ContainsKey(methodInfo.ReturnValue.TypeClass))
            //             //     {
            //             //         stringBuilder.Append($".As<{methodInfo.ReturnValue.GetTypeName()}>()");
            //             //     }
            //             //     else
            //             //     {
            //             //         stringBuilder.Append($".As<{returnValueName}>()");
            //             //     }
            //             // }
            //             stringBuilder.AppendLine(";");
            //     }
            //
            //     stringBuilder.AppendLine($"{TAB1}}}");
            //
            //     stringBuilder.AppendLine();
            //     stringBuilder
            //         .Append($"{TAB1}private")
            //         .Append(' ')
            //         .Append(returnValueName)
            //         .Append(' ')
            //         .Append(methodInfo.NativeName)
            //         .Append('(');
            //     BuildupMethodArguments(stringBuilder, methodInfo.Arguments, godotSharpTypeNameMap);
            //     stringBuilder.Append(')');
            //     stringBuilder.Append(
            //         $" => {virtualMethodName}({string.Join(',', methodInfo.Arguments.Select(a => a.GetArgumentName()))});");
            //     stringBuilder.AppendLine();
            // }
        }


        stringBuilder.AppendLine(
            """
            #endregion

            """
        );
    }

    private static void BuildupMethodCallArguments(
        StringBuilder stringBuilder,
        PropertyInfo[] propertyInfos,
        IReadOnlyDictionary<string, ClassInfo> gdeTypeMap,
        IReadOnlyDictionary<string, string> godotsharpTypeMap,
        ICollection<string> builtinTypes
    )
    {
        for (var i = 0; i < propertyInfos.Length; i++)
        {
            var propertyInfo = propertyInfos[i];

            var propertyTypeName = propertyInfo.GetTypeName();
            
            if (gdeTypeMap.TryGetValue(propertyTypeName, out var gdeClassInfo))
            {
                var bassType = GetEngineBaseType(gdeClassInfo, builtinTypes);
                bassType = godotsharpTypeMap.GetValueOrDefault(bassType, bassType);
                stringBuilder.Append($"({bassType})");
            }
            var argumentName = propertyInfo.GetArgumentName();
            
            if (propertyInfo.IsVoid && propertyInfo.Usage.HasFlag(PropertyUsageFlags.NilIsVariant))
            {
                stringBuilder
                    .Append(argumentName)
                    .Append(" ?? new Variant()");
            }
            else if (propertyInfo.IsEnum)
            {
                stringBuilder
                    .Append("Variant.From<")
                    .Append(propertyTypeName)
                    .Append(">(")
                    .Append(argumentName)
                    .Append(')');
            }
            else
            {
                stringBuilder.Append(argumentName);
            }
            
            
            if (i != propertyInfos.Length - 1)
            {
                stringBuilder.Append(", ");
            }
        }
    }
}
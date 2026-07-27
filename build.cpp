#include "helper.h"
#include "runner.h"
#include "ld.h"
#include "c.h"

#define UE4SS_DIR "loaders/ue4ss/"
DECLARE_BUILD_STAGE(ue4ss)
{
	CreateInterfaceFn pLibFPCFactory = Sys_GetFactory("fpc");
	ICCompiler *wincompiler = (ICCompiler*)pLibFPCFactory(MSVC_C_COMPILER_INTERFACE_NAME, NULL);
	ILinker *winlinker = (ILinker*)pLibFPCFactory(MSVC_LINKER_INTERFACE_NAME, NULL);

	LinkProject_t rust_ld = {};
	CUtlVector<CUtlString> args = {
		"build",
		"--examples",
		"--target",
		"x86_64-pc-windows-msvc",
	};
	runner->Run("cargo", args);
	runner->Wait();

	CProject_t cc = {};
	cc.m_szName = "main";
	cc.files =
	{
		UE4SS_DIR"brickrust/main.cpp",
		UE4SS_DIR"brickrust/interop.cpp",
	};
	cc.includeDirectories = 
	{
		UE4SS_DIR"RE-UE4SS/UE4SS/include",
		UE4SS_DIR"RE-UE4SS/deps/first/File/include",
		UE4SS_DIR"RE-UE4SS/deps/first/String/include",
		UE4SS_DIR"RE-UE4SS/deps/first/DynamicOutput/include",
		UE4SS_DIR"RE-UE4SS/deps/first/Unreal/include",
		UE4SS_DIR"RE-UE4SS/deps/first/Unreal/include/Unreal",
		UE4SS_DIR"RE-UE4SS/deps/first/Unreal/include/Unreal/Core",
		UE4SS_DIR"RE-UE4SS/deps/first/Unreal/generated_include",
		UE4SS_DIR"RE-UE4SS/deps/first/Constructs/include",
		UE4SS_DIR"RE-UE4SS/deps/first/Helpers/include",
		UE4SS_DIR"RE-UE4SS/deps/first/Function/include",
		UE4SS_DIR"RE-UE4SS/deps/first/ASMHelper/include",
		UE4SS_DIR"RE-UE4SS/deps/first/Input/include",
		UE4SS_DIR"RE-UE4SS/build_xwin/_deps/imguitextedit-src",
		UE4SS_DIR"RE-UE4SS/build_xwin/_deps/imgui-src",
		UE4SS_DIR"RE-UE4SS/build_xwin/_deps/zydis-src/include",
		UE4SS_DIR"RE-UE4SS/build_xwin/_deps/zydis-src/dependencies/zycore/include",
		UE4SS_DIR"RE-UE4SS/build_xwin/_deps/fmt-src/include",
	};
	cc.macros = {
		{"UBT_COMPILED_PLATFORM", "Windows"},
		{"UE_BUILD_DEVELOPMENT", "1"},
		{"PLATFORM_WINDOWS", "1"},
		{"WINVER", "0x100000"},
		{"UNICODE", "1"},
		{"_UNICODE", "1"},
	};

	cc.m_target = (Target_t){TARGET_KERNEL_WINDOWS, TARGET_CPU_AMD64, TARGET_ABI_MSVC};
	cc.m_target.eWindowsCRT = k_EWindowsCRT_Dynamic;
	cc.cppVersion = CPPVERSION_20; 
	LinkProject_t ld = wincompiler->Compile(&cc);
	ld.linkType = ELINK_DYNAMIC_LIBRARY;
	ld.libraryObjects = {
		"UE4SS.lib",
		"fmt.lib",
		"dbghelp.lib",
		"ws2_32.lib",
		"userenv.lib",
		"ntdll.lib",
	};
	ld.libraryDirectories = {
		UE4SS_DIR"RE-UE4SS/build_xwin/Game__Debug__Win64/lib"
	};
	if (CommandLine()->ParamValue("--example"))
	{
		CUtlString example = CUtlString("target/x86_64-pc-windows-msvc/debug/examples/%s.lib", CommandLine()->ParamValue("--example"));
		ld.objects.AppendTail({example});
	}

	CUtlString szOutput = winlinker->Link(&ld);
	ADD_OUTPUT_OBJECT("ue4ss", szOutput)

	return 0;
};

DECLARE_BUILD_STAGE(install)
{
	if (!CommandLine()->ParamValue("--install"))
		return 0;
	filesystem2->CopyFile(CommandLine()->ParamValue("--install"), GET_PROJECT_LIBRARY(ue4ss, "ue4ss"));
	return 0;
}

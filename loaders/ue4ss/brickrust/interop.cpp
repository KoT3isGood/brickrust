#include "DynamicOutput/DynamicOutput.hpp"
#include "Unreal/UObjectGlobals.hpp"
#include "Unreal/FString.hpp"
#include "Unreal/FText.hpp"
#include "Unreal/CoreUObject/UObject/Class.hpp"
#include "Unreal/CoreUObject/UObject/UnrealType.hpp"
#include "stdarg.h"
#include "stddef.h"
#include "string.h"
#include "stdlib.h"
#include "windows.h"

using namespace RC;
using namespace RC::Unreal;
extern "C"
{
	wchar_t *BrickRust_char_to_wchar( const char *c )
	{
		int size = MultiByteToWideChar(CP_UTF8, 0, c, -1, NULL, 0);
		wchar_t *wszName = (wchar_t*)malloc(size * sizeof(wchar_t));

		MultiByteToWideChar(CP_UTF8, 0, c, -1, wszName, size);
		return wszName;
	};
	char *BrickRust_wchar_to_char( const wchar_t *c )
	{
		int size = WideCharToMultiByte(CP_UTF8, 0, c, -1, NULL, 0, NULL, NULL);
		char* buffer = (char*)malloc(size);
		WideCharToMultiByte(CP_UTF8, 0, c, -1, buffer, size, NULL, NULL);
		return buffer;

	}
	void BrickRust_print( const char *c )
	{
		wchar_t *t = BrickRust_char_to_wchar(c);
		Output::send<LogLevel::Verbose>(STR("{}"), t);
		free(t);

	};
	void *BrickRust_FMalloc_Malloc( size_t s)
	{
		return (*GMalloc)->Malloc(s);
	}
	void *BrickRust_FMalloc_Realloc( void *p, size_t s )
	{
		return (*GMalloc)->Realloc(p, s);
	}
	void BrickRust_FMalloc_Free( void *p )
	{
		(*GMalloc)->Free(p);
	}
	bool BrickRust_FMalloc_GetAllocationSize( void *p, size_t *size_out )
	{
		size_t s;
		bool b = (*GMalloc)->GetAllocationSize(p,s);
		if (b)
			*size_out = s;
		return b;
	}

	void UE4SS_fname_to_string( FName *name, FString *string )
	{
		*string = name->ToFString();
	}
	void UE4SS_ftext_to_string( FText *text, FString *string )
	{
		*string = text->ToFString();
	};
	void UE4SS_string_to_fname( const char *str, FName *name )
	{
		int size = MultiByteToWideChar(CP_UTF8, 0, str, -1, NULL, 0);
		wchar_t *wszName = (wchar_t*)malloc(size * sizeof(wchar_t));

		MultiByteToWideChar(CP_UTF8, 0, str, -1, wszName, size);

		*name = FName(wszName);

		free(wszName);
	}
	void UE4SS_string_to_ftext( const char *str, FText *text )
	{
		int size = MultiByteToWideChar(CP_UTF8, 0, str, -1, NULL, 0);
		wchar_t *wszName = (wchar_t*)malloc(size * sizeof(wchar_t));

		MultiByteToWideChar(CP_UTF8, 0, str, -1, wszName, size);

		*text = FText(wszName);

		free(wszName);
	}
	FProperty *UE4SS_get_property_by_name( UObject *pObject, const char *szName )
	{
		int size = MultiByteToWideChar(CP_UTF8, 0, szName, -1, NULL, 0);
		wchar_t *wszName = (wchar_t*)malloc(size * sizeof(wchar_t));

		MultiByteToWideChar(CP_UTF8, 0, szName, -1, wszName, size);

		UClass* cls = pObject->GetClassPrivate();

		for (FProperty* prop = cls->GetPropertyLink(); prop; prop = prop->GetPropertyLinkNext())
		{
			if (!wcscmp(prop->GetName().c_str(), wszName))
			{
				free(wszName);
				return prop;
			}
		}
		free(wszName);
		return NULL;

	};
	const UStruct *BrickRust_GetUStructFromName( const char *child, const char *parent )
	{
		auto child_name = BrickRust_char_to_wchar(child+1);
		auto parent_name = BrickRust_char_to_wchar(parent);
		auto FullName = fmt::format(STR("/Script/{}.{}"), parent_name, child_name);
		return static_cast<UStruct*>(UObjectGlobals::StaticFindObject_InternalSlow(nullptr, nullptr, FullName.c_str()));
	}

	void *UE4SS_get_value_by_name( UObject *pObject, const char *szName )
	{
		FProperty *pProp = UE4SS_get_property_by_name( pObject, szName );
		if (pProp)
			return pProp->ContainerPtrToValuePtr<void>(pObject);
		else
			return NULL;
	}
	char* UE4SS_string_to_utf8( FString *str )
	{
		int size_needed = WideCharToMultiByte(
				CP_UTF8,
				0,
				str->GetCharArray().GetData(),
				str->GetCharArray().Num(),
				NULL,
				0,
				NULL,
				NULL
				);

		if (size_needed <= 0)
			return NULL;

		char* utf8 = (char*)malloc(size_needed+1);
		if (!utf8)
			return NULL;

		int result = WideCharToMultiByte(
				CP_UTF8,
				0,
				str->GetCharArray().GetData(),
				str->GetCharArray().Num(),
				utf8,
				size_needed,
				NULL,
				NULL
				);

		if (result == 0) {
			free(utf8);
			return NULL;
		}

		utf8[size_needed] = 0;
		return utf8;
	}

	void *UE4SS_find_native_function( UObject *pObject, const char *szName )
	{
		int size = MultiByteToWideChar(CP_UTF8, 0, szName, -1, NULL, 0);
		wchar_t *wszName = (wchar_t*)malloc(size * sizeof(wchar_t));

		MultiByteToWideChar(CP_UTF8, 0, szName, -1, wszName, size);
		
		UFunction *pFunc = NULL;

		UClass* cls = pObject->GetClassPrivate();
		while (cls)
		{
			for (auto a: cls->GetFuncMap())
			{
				if (wcscmp(a.Key.ToString().c_str(), wszName))
					continue;
				pFunc = a.Value;
				goto found;
			}
			cls = cls->GetSuperClass();
		}
found:
		if (pFunc)
			return (void*)pFunc->GetFuncPtr();
		else
			return NULL;
	}

	void UE4SS_call_function( UObject *pObject, const char *szName, void *pParams )
	{
		int size = MultiByteToWideChar(CP_UTF8, 0, szName, -1, NULL, 0);
		wchar_t *wszName = (wchar_t*)malloc(size * sizeof(wchar_t));

		MultiByteToWideChar(CP_UTF8, 0, szName, -1, wszName, size);
		
		UFunction *pFunc = NULL;

		UClass* cls = pObject->GetClassPrivate();
		while (cls)
		{
			for (auto a: cls->GetFuncMap())
			{
				if (wcscmp(a.Key.ToString().c_str(), wszName))
					continue;
				pFunc = a.Value;
				goto found;
			}
			cls = cls->GetSuperClass();
		}
found:
		/*
		for (FProperty* prop = pFunc->GetPropertyLink(); prop; prop = prop->GetPropertyLinkNext())
		{
			RC::Output::send<RC::LogLevel::Verbose>(STR("{} {} {} {}\n"), prop->GetOffset_Internal(), prop->GetSize(), prop->GetCPPType().GetCharArray().GetData(), prop->GetName().c_str());
		}
		*/
	
		if (pFunc)
			pObject->ProcessEvent(pFunc, pParams);

		free(wszName);
	}
}

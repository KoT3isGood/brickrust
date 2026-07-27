#include "stdio.h"
#include <Mod/CppUserModBase.hpp>
#include "Unreal/Hooks.hpp"
#include "Unreal/CoreUObject/UObject/Class.hpp"

using namespace RC;
using namespace RC::Unreal;
void __nop() {};

class UBrick : public UObject
{
	DECLARE_EXTERNAL_OBJECT_CLASS(UBrick, BrickRigs);
};
IMPLEMENT_EXTERNAL_OBJECT_CLASS(UBrick);

extern "C"
{

	struct ModInfo
	{
		const char *name;
		const char *description;
		const char *version;
		const char *authors;
	};

	ModInfo brickrigs_mod_info();
	void brickrigs_init();
	void brickrigs_frame();
	void brickrigs_deinit();
	void brickrigs_on_brick_created( UBrick *obj );
	wchar_t *BrickRust_char_to_wchar( const char *c );
	char *BrickRust_wchar_to_char( const wchar_t *c );
}


class BrickRustMod : public RC::CppUserModBase
{
public:
	BrickRustMod() : CppUserModBase()
	{
		ModInfo i = brickrigs_mod_info();
		ModName = BrickRust_char_to_wchar(i.name);
		ModVersion = BrickRust_char_to_wchar(i.version);
		ModDescription = BrickRust_char_to_wchar(i.description);
		ModAuthors = BrickRust_char_to_wchar(i.authors);
	}

	~BrickRustMod() override
	{
		brickrigs_deinit();
	}

        virtual void on_unreal_init() override
	{
		brickrigs_init();

		Unreal::Hook::RegisterStaticConstructObjectPostCallback([](
			const Unreal::FStaticConstructObjectParameters& params,
			Unreal::UObject* obj
		) -> Unreal::UObject* {
			
			
			if (obj->IsA<UBrick>())
			{
				brickrigs_on_brick_created((UBrick*)obj);
			}
			return NULL;

		} );
	}

	auto on_update() -> void override
	{
		brickrigs_frame();
	}
};

#define BRICKRUST_API __declspec(dllexport)
extern "C"
{
	BRICKRUST_API RC::CppUserModBase* start_mod()
	{
		return new BrickRustMod();
	}

	BRICKRUST_API void uninstall_mod(RC::CppUserModBase* mod)
	{
		delete mod;
	}
}

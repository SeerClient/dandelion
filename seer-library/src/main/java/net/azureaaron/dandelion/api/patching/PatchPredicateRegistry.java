package net.azureaaron.dandelion.api.patching;

import java.util.Objects;

import com.mojang.serialization.MapCodec;

import net.azureaaron.dandelion.impl.patching.ConfigPatcher;
import net.minecraft.resources.Identifier;

/**
 * Allows users of Dandelion to register their own custom {@link PatchPredicate}s.
 */
public final class PatchPredicateRegistry {

	/**
	 * Registers a custom {@link PatchPredicate}.
	 *
	 * @param id    the predicate's id
	 * @param codec the predicate's {@link MapCodec}
	 */
	public static void register(Identifier id, MapCodec<? extends PatchPredicate> codec) {
		ConfigPatcher.PREDICATE_ID_MAPPER.put(Objects.requireNonNull(id, "id must not be null"), Objects.requireNonNull(codec, "codec must not be null"));
	}
}

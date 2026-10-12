package net.azureaaron.dandelion.api.patching;

import java.util.OptionalLong;

import com.mojang.serialization.MapCodec;

import net.azureaaron.dandelion.Dandelion;
import net.fabricmc.loader.api.SemanticVersion;
import net.minecraft.resources.Identifier;

/**
 * A predicate that will be used in determining whether a {@link ConfigPatch} should be applied.
 */
public interface PatchPredicate {

	/**
	 * {@return the {@link MapCodec} used to deserialize the predicate}
	 */
	MapCodec<? extends PatchPredicate> codec();

	/**
	 * Evaluates the predicate.
	 *
	 * @return {@code true} if the predicate's condition was satisfied, otherwise {@code false}
	 */
	boolean test();

	/**
	 * The built-in date predicate.
	 *
	 * @apiNote Do not extend this.
	 */
	interface OfDate extends PatchPredicate {
		Identifier ID = Dandelion.id("predicate/date");

		/**
		 * {@return the start date}
		 */
		OptionalLong from();

		/**
		 * {@return the end date}
		 */
		OptionalLong to();
	}

	/**
	 * The built-in Minecraft version predicate.
	 *
	 * @apiNote Do not extend this.
	 */
	interface OfMinecraftVersion extends PatchPredicate {
		Identifier ID = Dandelion.id("predicate/minecraft_version");

		/**
		 * {@return the version of the Minecraft to compare against}
		 */
		String version();

		/**
		 * {@return the operator used to compare the current Minecraft version against the given Minecraft version}
		 */
		ComparisonOperator operator();
	}

	/**
	 * The built-in mod version predicate. The mod must use semver version formatting.
	 *
	 * @apiNote Do not extend this.
	 */
	interface OfModVersion extends PatchPredicate {
		Identifier ID = Dandelion.id("predicate/mod_version");

		/**
		 * {@return the id of the mod whose version will be compared}
		 */
		String modId();

		/**
		 * {@return the version of the mod to compare against}
		 */
		SemanticVersion version();

		/**
		 * {@return the operator used to compare the current mod version against the given mod version}
		 */
		ComparisonOperator operator();
	}
}

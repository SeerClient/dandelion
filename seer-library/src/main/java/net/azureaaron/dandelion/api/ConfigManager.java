package net.azureaaron.dandelion.api;

import java.nio.file.Path;
import java.util.List;
import java.util.function.UnaryOperator;

import com.google.gson.GsonBuilder;

import net.azureaaron.dandelion.api.patching.ConfigPatch;
import net.azureaaron.dandelion.impl.ConfigManagerImpl;

public interface ConfigManager<T> {

	static <T> ConfigManager<T> create(Class<T> configClass, Path path, UnaryOperator<GsonBuilder> gsonBuilder) {
		return new ConfigManagerImpl<>(configClass, path, gsonBuilder);
	}

	/**
	 * {@return the class backing this config}
	 */
	Class<T> configClass();

	/**
	 * Returns the patched instance of the {@code configClass} or a default instance if the config hasn't been successfully loaded.
	 *
	 * <p>The underlying instance may change at any time, retaining strong references to it is highly discouraged.
	 *
	 * <p>Note: <strong>This instance is NEVER saved as the user's config! Do not write updated values to it.</strong>
	 */
	T instance();

	/**
	 * Returns the unpatched instance of the {@code configClass} or a default instance if the config hasn't been successfully loaded.
	 *
	 * <p>The underlying instance may change at any time, retaining strong references to it is highly discouraged.
	 */
	T unpatchedInstance();

	/**
	 * {@return an instance of the {@code configClass} representing the default initialized values}
	 */
	T defaults();

	/**
	 * {@return whether the config saved successfully}
	 */
	boolean save();

	/**
	 * {@return whether the config loaded successfully}
	 */
	boolean load();

	/**
	 * Updates the patched instance such that all its values match the unpatched instance.
	 */
	void updatePatchedInstance();

	/**
	 * Replaces the existing set of config patches (if any) and then applies them.
	 */
	void setPatches(List<ConfigPatch> patches);
}

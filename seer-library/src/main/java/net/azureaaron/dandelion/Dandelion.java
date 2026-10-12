package net.azureaaron.dandelion;

import net.fabricmc.api.ClientModInitializer;
import net.minecraft.resources.Identifier;

public class Dandelion implements ClientModInitializer {

	@Override
	public void onInitializeClient() {}

	public static Identifier id(String path) {
		return Identifier.fromNamespaceAndPath("dandelion", path);
	}
}

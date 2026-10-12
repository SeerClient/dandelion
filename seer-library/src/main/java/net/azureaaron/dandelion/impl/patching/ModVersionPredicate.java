package net.azureaaron.dandelion.impl.patching;

import java.util.Optional;

import com.mojang.serialization.Codec;
import com.mojang.serialization.MapCodec;
import com.mojang.serialization.codecs.RecordCodecBuilder;

import net.azureaaron.dandelion.api.patching.ComparisonOperator;
import net.azureaaron.dandelion.api.patching.PatchPredicate;
import net.azureaaron.dandelion.impl.utils.CodecUtils;
import net.fabricmc.loader.api.FabricLoader;
import net.fabricmc.loader.api.ModContainer;
import net.fabricmc.loader.api.SemanticVersion;
import net.fabricmc.loader.api.Version;

public record ModVersionPredicate(String modId, SemanticVersion version, ComparisonOperator operator) implements PatchPredicate.OfModVersion {
	protected static final MapCodec<ModVersionPredicate> MAP_CODEC = RecordCodecBuilder.mapCodec(instance -> instance.group(
			Codec.STRING.fieldOf("modId").forGetter(ModVersionPredicate::modId),
			CodecUtils.SEMANTIC_VERSION_CODEC.fieldOf("version").forGetter(ModVersionPredicate::version),
			ComparisonOperator.CODEC.fieldOf("operator").forGetter(ModVersionPredicate::operator))
			.apply(instance, ModVersionPredicate::new));

	@Override
	public MapCodec<? extends PatchPredicate> codec() {
		return MAP_CODEC;
	}

	@Override
	public boolean test() {
		Optional<ModContainer> modContainer = FabricLoader.getInstance().getModContainer(this.modId());

		if (modContainer.isPresent()) {
			Version currentVersion = modContainer.get().getMetadata().getVersion();

			return this.operator().compare(currentVersion, this.version());
		}

		return false;
	}
}

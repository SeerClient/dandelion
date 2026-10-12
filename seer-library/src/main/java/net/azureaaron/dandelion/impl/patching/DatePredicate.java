package net.azureaaron.dandelion.impl.patching;

import java.util.OptionalLong;

import com.mojang.serialization.Codec;
import com.mojang.serialization.MapCodec;
import com.mojang.serialization.codecs.RecordCodecBuilder;

import net.azureaaron.dandelion.api.patching.PatchPredicate;
import net.minecraft.util.ExtraCodecs;

public record DatePredicate(OptionalLong from, OptionalLong to) implements PatchPredicate.OfDate {
	protected static final MapCodec<DatePredicate> MAP_CODEC = RecordCodecBuilder.mapCodec(instance -> instance.group(
			ExtraCodecs.asOptionalLong(Codec.LONG.optionalFieldOf("from")).forGetter(DatePredicate::from),
			ExtraCodecs.asOptionalLong(Codec.LONG.optionalFieldOf("to")).forGetter(DatePredicate::to))
			.apply(instance, DatePredicate::new));

	@Override
	public MapCodec<? extends PatchPredicate> codec() {
		return MAP_CODEC;
	}

	@Override
	public boolean test() {
		boolean result = true;
		long now = System.currentTimeMillis();

		if (this.from().isPresent()) {
			result &= now >= this.from().getAsLong();
		}

		if (this.to().isPresent()) {
			result &= now <= this.to().getAsLong();
		}

		return result;
	}
}

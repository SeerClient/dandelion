package net.azureaaron.dandelion.api.patching;

import com.mojang.serialization.Codec;

import net.fabricmc.loader.api.Version;
import net.minecraft.util.StringRepresentable;

/**
 * Enumeration of all comparison operations.
 */
public enum ComparisonOperator implements StringRepresentable {
	EQUAL_TO,
	NOT_EQUAL_TO,
	GREATER_THAN,
	GREATER_THAN_OR_EQUAL_TO,
	LESS_THAN,
	LESS_THAN_OR_EQUAL_TO;

	public static final Codec<ComparisonOperator> CODEC = StringRepresentable.fromEnum(ComparisonOperator::values);

	@Override
	public String getSerializedName() {
		return this.name();
	}

	public boolean compare(Version version1, Version version2) {
		return switch (this) {
			case EQUAL_TO -> version1.compareTo(version2) == 0;
			case NOT_EQUAL_TO -> version1.compareTo(version2) != 0;
			case GREATER_THAN -> version1.compareTo(version2) > 0;
			case GREATER_THAN_OR_EQUAL_TO -> version1.compareTo(version2) >= 0;
			case LESS_THAN -> version1.compareTo(version2) < 0;
			case LESS_THAN_OR_EQUAL_TO -> version1.compareTo(version2) <= 0;
		};
	}
}

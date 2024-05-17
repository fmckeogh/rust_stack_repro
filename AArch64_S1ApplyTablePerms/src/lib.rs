#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use EL2Enabled::*;
use HasUnprivileged::*;
use common::*;
pub fn AArch64_S1ApplyTablePerms<T: Tracer>(
    state: &mut State,
    tracer: &T,
    permissions_in: ProductTypebf05c51f33174538,
    descriptor: Bits,
    regime: u32,
    walkparams: ProductTypeef284266e139aee2,
) -> ProductTypebf05c51f33174538 {
    #[derive(Default)]
    struct FunctionState {
        xn_table: bool,
        pxn_table: bool,
        gs_18128: bool,
        gs_18127: bool,
        permissions: ProductTypebf05c51f33174538,
        ap_table: u8,
        uxn_table: bool,
        permissions_in: ProductTypebf05c51f33174538,
        descriptor: Bits,
        regime: u32,
        walkparams: ProductTypeef284266e139aee2,
    }
    let fn_state = FunctionState {
        permissions_in,
        descriptor,
        regime,
        walkparams,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_0_0: read-var permissions_in:struct
        let s_0_0: ProductTypebf05c51f33174538 = fn_state.permissions_in;
        // D s_0_1: write-var permissions <= s_0_0
        fn_state.permissions = s_0_0;
        // D s_0_2: read-var regime:u32
        let s_0_2: u32 = fn_state.regime;
        // C s_0_3: const #4u : u32
        let s_0_3: u32 = 4;
        // D s_0_4: cmp-eq s_0_2 s_0_3
        let s_0_4: bool = ((s_0_2) == (s_0_3));
        // N s_0_5: branch s_0_4 b20 b1
        if s_0_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#18127 <= s_1_0
        fn_state.gs_18127 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_2_0: read-var gs#18127:u8
        let s_2_0: bool = fn_state.gs_18127;
        // N s_2_1: branch s_2_0 b19 b3
        if s_2_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#18128 <= s_3_0
        fn_state.gs_18128 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_4_0: read-var gs#18128:u8
        let s_4_0: bool = fn_state.gs_18128;
        // N s_4_1: branch s_4_0 b15 b5
        if s_4_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_5_0: read-var regime:u32
        let s_5_0: u32 = fn_state.regime;
        // D s_5_1: call HasUnprivileged(s_5_0)
        let s_5_1: bool = HasUnprivileged(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b11 b6
        if s_5_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_6_0: read-var walkparams.3:struct
        let s_6_0: bool = fn_state.walkparams._3;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b10 b7
        if s_6_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_7_0: read-var descriptor:bv
        let s_7_0: Bits = fn_state.descriptor;
        // D s_7_1: size-of s_7_0
        let s_7_1: u16 = s_7_0.length();
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // C s_7_3: const #62s : i
        let s_7_3: i128 = 62;
        // D s_7_4: cmp-lt s_7_3 s_7_2
        let s_7_4: bool = ((s_7_3) < (s_7_2));
        // N s_7_5: assert s_7_4
        let s_7_5: () = assert!(s_7_4);
        // C s_7_6: const #62s : i
        let s_7_6: i128 = 62;
        // D s_7_7: read-var descriptor:bv
        let s_7_7: Bits = fn_state.descriptor;
        // C s_7_8: const #1u : u64
        let s_7_8: u64 = 1;
        // D s_7_9: bit-extract s_7_7 s_7_6 s_7_8
        let s_7_9: Bits = (Bits::new(
            ((s_7_7) >> (s_7_6)).value(),
            u16::try_from(s_7_8).unwrap(),
        ));
        // D s_7_10: cast reint s_7_9 -> u8
        let s_7_10: bool = ((s_7_9.value()) != 0);
        // C s_7_11: const #0s : i
        let s_7_11: i128 = 0;
        // C s_7_12: const #0u : u64
        let s_7_12: u64 = 0;
        // D s_7_13: cast zx s_7_10 -> u64
        let s_7_13: u64 = (s_7_10 as u64);
        // C s_7_14: const #1u : u64
        let s_7_14: u64 = 1;
        // D s_7_15: and s_7_13 s_7_14
        let s_7_15: u64 = ((s_7_13) & (s_7_14));
        // D s_7_16: cmp-eq s_7_15 s_7_14
        let s_7_16: bool = ((s_7_15) == (s_7_14));
        // D s_7_17: lsl s_7_13 s_7_11
        let s_7_17: u64 = s_7_13 << s_7_11;
        // D s_7_18: or s_7_12 s_7_17
        let s_7_18: u64 = ((s_7_12) | (s_7_17));
        // D s_7_19: cmpl s_7_17
        let s_7_19: u64 = !s_7_17;
        // D s_7_20: and s_7_12 s_7_19
        let s_7_20: u64 = ((s_7_12) & (s_7_19));
        // D s_7_21: select s_7_16 s_7_18 s_7_20
        let s_7_21: u64 = if s_7_16 { s_7_18 } else { s_7_20 };
        // D s_7_22: cast trunc s_7_21 -> u8
        let s_7_22: bool = ((s_7_21) != 0);
        // D s_7_23: cast zx s_7_22 -> bv
        let s_7_23: Bits = Bits::new(s_7_22 as u128, 1u16);
        // C s_7_24: const #0u : u8
        let s_7_24: bool = false;
        // C s_7_25: cast zx s_7_24 -> bv
        let s_7_25: Bits = Bits::new(s_7_24 as u128, 1u16);
        // D s_7_26: cast reint s_7_23 -> u128
        let s_7_26: u128 = (s_7_23.value() as u128);
        // D s_7_27: size-of s_7_23
        let s_7_27: u16 = s_7_23.length();
        // C s_7_28: cast reint s_7_25 -> u128
        let s_7_28: u128 = (s_7_25.value() as u128);
        // D s_7_29: size-of s_7_25
        let s_7_29: u16 = s_7_25.length();
        // D s_7_30: lsl s_7_26 s_7_29
        let s_7_30: u128 = s_7_26 << s_7_29;
        // D s_7_31: or s_7_30 s_7_28
        let s_7_31: u128 = ((s_7_30) | (s_7_28));
        // D s_7_32: add s_7_27 s_7_29
        let s_7_32: u16 = (s_7_27 + s_7_29);
        // D s_7_33: create-bits s_7_31 s_7_32
        let s_7_33: Bits = Bits::new(s_7_31, s_7_32);
        // D s_7_34: cast reint s_7_33 -> u8
        let s_7_34: u8 = (s_7_33.value() as u8);
        // D s_7_35: write-var ap_table <= s_7_34
        fn_state.ap_table = s_7_34;
        // C s_7_36: const #60s : i
        let s_7_36: i128 = 60;
        // D s_7_37: read-var descriptor:bv
        let s_7_37: Bits = fn_state.descriptor;
        // C s_7_38: const #1u : u64
        let s_7_38: u64 = 1;
        // D s_7_39: bit-extract s_7_37 s_7_36 s_7_38
        let s_7_39: Bits = (Bits::new(
            ((s_7_37) >> (s_7_36)).value(),
            u16::try_from(s_7_38).unwrap(),
        ));
        // D s_7_40: cast reint s_7_39 -> u8
        let s_7_40: bool = ((s_7_39.value()) != 0);
        // C s_7_41: const #0s : i
        let s_7_41: i128 = 0;
        // C s_7_42: const #0u : u64
        let s_7_42: u64 = 0;
        // D s_7_43: cast zx s_7_40 -> u64
        let s_7_43: u64 = (s_7_40 as u64);
        // C s_7_44: const #1u : u64
        let s_7_44: u64 = 1;
        // D s_7_45: and s_7_43 s_7_44
        let s_7_45: u64 = ((s_7_43) & (s_7_44));
        // D s_7_46: cmp-eq s_7_45 s_7_44
        let s_7_46: bool = ((s_7_45) == (s_7_44));
        // D s_7_47: lsl s_7_43 s_7_41
        let s_7_47: u64 = s_7_43 << s_7_41;
        // D s_7_48: or s_7_42 s_7_47
        let s_7_48: u64 = ((s_7_42) | (s_7_47));
        // D s_7_49: cmpl s_7_47
        let s_7_49: u64 = !s_7_47;
        // D s_7_50: and s_7_42 s_7_49
        let s_7_50: u64 = ((s_7_42) & (s_7_49));
        // D s_7_51: select s_7_46 s_7_48 s_7_50
        let s_7_51: u64 = if s_7_46 { s_7_48 } else { s_7_50 };
        // D s_7_52: cast trunc s_7_51 -> u8
        let s_7_52: bool = ((s_7_51) != 0);
        // D s_7_53: write-var xn_table <= s_7_52
        fn_state.xn_table = s_7_52;
        // N s_7_54: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_8_0: read-var permissions.1:struct
        let s_8_0: u8 = fn_state.permissions._1;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // D s_8_2: read-var ap_table:u8
        let s_8_2: u8 = fn_state.ap_table;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: or s_8_1 s_8_3
        let s_8_4: Bits = ((s_8_1) | (s_8_3));
        // D s_8_5: cast reint s_8_4 -> u8
        let s_8_5: u8 = (s_8_4.value() as u8);
        // D s_8_6: write-var permissions.1 <= s_8_5
        fn_state.permissions._1 = s_8_5;
        // D s_8_7: read-var permissions.18:struct
        let s_8_7: bool = fn_state.permissions._18;
        // D s_8_8: cast zx s_8_7 -> bv
        let s_8_8: Bits = Bits::new(s_8_7 as u128, 1u16);
        // D s_8_9: read-var xn_table:u8
        let s_8_9: bool = fn_state.xn_table;
        // D s_8_10: cast zx s_8_9 -> bv
        let s_8_10: Bits = Bits::new(s_8_9 as u128, 1u16);
        // D s_8_11: or s_8_8 s_8_10
        let s_8_11: Bits = ((s_8_8) | (s_8_10));
        // D s_8_12: cast reint s_8_11 -> u8
        let s_8_12: bool = ((s_8_11.value()) != 0);
        // D s_8_13: write-var permissions.18 <= s_8_12
        fn_state.permissions._18 = s_8_12;
        // N s_8_14: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_9_0: read-var permissions:struct
        let s_9_0: ProductTypebf05c51f33174538 = fn_state.permissions;
        // N s_9_1: return s_9_0
        return s_9_0;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_10_0: read-var descriptor:bv
        let s_10_0: Bits = fn_state.descriptor;
        // D s_10_1: size-of s_10_0
        let s_10_1: u16 = s_10_0.length();
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // C s_10_3: const #126s : i
        let s_10_3: i128 = 126;
        // D s_10_4: cmp-lt s_10_3 s_10_2
        let s_10_4: bool = ((s_10_3) < (s_10_2));
        // N s_10_5: assert s_10_4
        let s_10_5: () = assert!(s_10_4);
        // C s_10_6: const #126s : i
        let s_10_6: i128 = 126;
        // D s_10_7: read-var descriptor:bv
        let s_10_7: Bits = fn_state.descriptor;
        // C s_10_8: const #1u : u64
        let s_10_8: u64 = 1;
        // D s_10_9: bit-extract s_10_7 s_10_6 s_10_8
        let s_10_9: Bits = (Bits::new(
            ((s_10_7) >> (s_10_6)).value(),
            u16::try_from(s_10_8).unwrap(),
        ));
        // D s_10_10: cast reint s_10_9 -> u8
        let s_10_10: bool = ((s_10_9.value()) != 0);
        // C s_10_11: const #0s : i
        let s_10_11: i128 = 0;
        // C s_10_12: const #0u : u64
        let s_10_12: u64 = 0;
        // D s_10_13: cast zx s_10_10 -> u64
        let s_10_13: u64 = (s_10_10 as u64);
        // C s_10_14: const #1u : u64
        let s_10_14: u64 = 1;
        // D s_10_15: and s_10_13 s_10_14
        let s_10_15: u64 = ((s_10_13) & (s_10_14));
        // D s_10_16: cmp-eq s_10_15 s_10_14
        let s_10_16: bool = ((s_10_15) == (s_10_14));
        // D s_10_17: lsl s_10_13 s_10_11
        let s_10_17: u64 = s_10_13 << s_10_11;
        // D s_10_18: or s_10_12 s_10_17
        let s_10_18: u64 = ((s_10_12) | (s_10_17));
        // D s_10_19: cmpl s_10_17
        let s_10_19: u64 = !s_10_17;
        // D s_10_20: and s_10_12 s_10_19
        let s_10_20: u64 = ((s_10_12) & (s_10_19));
        // D s_10_21: select s_10_16 s_10_18 s_10_20
        let s_10_21: u64 = if s_10_16 { s_10_18 } else { s_10_20 };
        // D s_10_22: cast trunc s_10_21 -> u8
        let s_10_22: bool = ((s_10_21) != 0);
        // D s_10_23: cast zx s_10_22 -> bv
        let s_10_23: Bits = Bits::new(s_10_22 as u128, 1u16);
        // C s_10_24: const #0u : u8
        let s_10_24: bool = false;
        // C s_10_25: cast zx s_10_24 -> bv
        let s_10_25: Bits = Bits::new(s_10_24 as u128, 1u16);
        // D s_10_26: cast reint s_10_23 -> u128
        let s_10_26: u128 = (s_10_23.value() as u128);
        // D s_10_27: size-of s_10_23
        let s_10_27: u16 = s_10_23.length();
        // C s_10_28: cast reint s_10_25 -> u128
        let s_10_28: u128 = (s_10_25.value() as u128);
        // D s_10_29: size-of s_10_25
        let s_10_29: u16 = s_10_25.length();
        // D s_10_30: lsl s_10_26 s_10_29
        let s_10_30: u128 = s_10_26 << s_10_29;
        // D s_10_31: or s_10_30 s_10_28
        let s_10_31: u128 = ((s_10_30) | (s_10_28));
        // D s_10_32: add s_10_27 s_10_29
        let s_10_32: u16 = (s_10_27 + s_10_29);
        // D s_10_33: create-bits s_10_31 s_10_32
        let s_10_33: Bits = Bits::new(s_10_31, s_10_32);
        // D s_10_34: cast reint s_10_33 -> u8
        let s_10_34: u8 = (s_10_33.value() as u8);
        // D s_10_35: write-var ap_table <= s_10_34
        fn_state.ap_table = s_10_34;
        // C s_10_36: const #124s : i
        let s_10_36: i128 = 124;
        // D s_10_37: read-var descriptor:bv
        let s_10_37: Bits = fn_state.descriptor;
        // C s_10_38: const #1u : u64
        let s_10_38: u64 = 1;
        // D s_10_39: bit-extract s_10_37 s_10_36 s_10_38
        let s_10_39: Bits = (Bits::new(
            ((s_10_37) >> (s_10_36)).value(),
            u16::try_from(s_10_38).unwrap(),
        ));
        // D s_10_40: cast reint s_10_39 -> u8
        let s_10_40: bool = ((s_10_39.value()) != 0);
        // C s_10_41: const #0s : i
        let s_10_41: i128 = 0;
        // C s_10_42: const #0u : u64
        let s_10_42: u64 = 0;
        // D s_10_43: cast zx s_10_40 -> u64
        let s_10_43: u64 = (s_10_40 as u64);
        // C s_10_44: const #1u : u64
        let s_10_44: u64 = 1;
        // D s_10_45: and s_10_43 s_10_44
        let s_10_45: u64 = ((s_10_43) & (s_10_44));
        // D s_10_46: cmp-eq s_10_45 s_10_44
        let s_10_46: bool = ((s_10_45) == (s_10_44));
        // D s_10_47: lsl s_10_43 s_10_41
        let s_10_47: u64 = s_10_43 << s_10_41;
        // D s_10_48: or s_10_42 s_10_47
        let s_10_48: u64 = ((s_10_42) | (s_10_47));
        // D s_10_49: cmpl s_10_47
        let s_10_49: u64 = !s_10_47;
        // D s_10_50: and s_10_42 s_10_49
        let s_10_50: u64 = ((s_10_42) & (s_10_49));
        // D s_10_51: select s_10_46 s_10_48 s_10_50
        let s_10_51: u64 = if s_10_46 { s_10_48 } else { s_10_50 };
        // D s_10_52: cast trunc s_10_51 -> u8
        let s_10_52: bool = ((s_10_51) != 0);
        // D s_10_53: write-var xn_table <= s_10_52
        fn_state.xn_table = s_10_52;
        // N s_10_54: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_11_0: read-var walkparams.3:struct
        let s_11_0: bool = fn_state.walkparams._3;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b14 b12
        if s_11_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_12_0: read-var descriptor:bv
        let s_12_0: Bits = fn_state.descriptor;
        // D s_12_1: size-of s_12_0
        let s_12_1: u16 = s_12_0.length();
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // C s_12_3: const #62s : i
        let s_12_3: i128 = 62;
        // D s_12_4: cmp-lt s_12_3 s_12_2
        let s_12_4: bool = ((s_12_3) < (s_12_2));
        // N s_12_5: assert s_12_4
        let s_12_5: () = assert!(s_12_4);
        // C s_12_6: const #61s : i
        let s_12_6: i128 = 61;
        // D s_12_7: read-var descriptor:bv
        let s_12_7: Bits = fn_state.descriptor;
        // C s_12_8: const #1s : i64
        let s_12_8: i64 = 1;
        // C s_12_9: cast zx s_12_8 -> i
        let s_12_9: i128 = (i128::try_from(s_12_8).unwrap());
        // C s_12_10: const #1s : i
        let s_12_10: i128 = 1;
        // C s_12_11: add s_12_10 s_12_9
        let s_12_11: i128 = (s_12_10 + s_12_9);
        // D s_12_12: bit-extract s_12_7 s_12_6 s_12_11
        let s_12_12: Bits = (Bits::new(
            ((s_12_7) >> (s_12_6)).value(),
            u16::try_from(s_12_11).unwrap(),
        ));
        // D s_12_13: cast reint s_12_12 -> u8
        let s_12_13: u8 = (s_12_12.value() as u8);
        // D s_12_14: write-var ap_table <= s_12_13
        fn_state.ap_table = s_12_13;
        // C s_12_15: const #60s : i
        let s_12_15: i128 = 60;
        // D s_12_16: read-var descriptor:bv
        let s_12_16: Bits = fn_state.descriptor;
        // C s_12_17: const #1u : u64
        let s_12_17: u64 = 1;
        // D s_12_18: bit-extract s_12_16 s_12_15 s_12_17
        let s_12_18: Bits = (Bits::new(
            ((s_12_16) >> (s_12_15)).value(),
            u16::try_from(s_12_17).unwrap(),
        ));
        // D s_12_19: cast reint s_12_18 -> u8
        let s_12_19: bool = ((s_12_18.value()) != 0);
        // C s_12_20: const #0s : i
        let s_12_20: i128 = 0;
        // C s_12_21: const #0u : u64
        let s_12_21: u64 = 0;
        // D s_12_22: cast zx s_12_19 -> u64
        let s_12_22: u64 = (s_12_19 as u64);
        // C s_12_23: const #1u : u64
        let s_12_23: u64 = 1;
        // D s_12_24: and s_12_22 s_12_23
        let s_12_24: u64 = ((s_12_22) & (s_12_23));
        // D s_12_25: cmp-eq s_12_24 s_12_23
        let s_12_25: bool = ((s_12_24) == (s_12_23));
        // D s_12_26: lsl s_12_22 s_12_20
        let s_12_26: u64 = s_12_22 << s_12_20;
        // D s_12_27: or s_12_21 s_12_26
        let s_12_27: u64 = ((s_12_21) | (s_12_26));
        // D s_12_28: cmpl s_12_26
        let s_12_28: u64 = !s_12_26;
        // D s_12_29: and s_12_21 s_12_28
        let s_12_29: u64 = ((s_12_21) & (s_12_28));
        // D s_12_30: select s_12_25 s_12_27 s_12_29
        let s_12_30: u64 = if s_12_25 { s_12_27 } else { s_12_29 };
        // D s_12_31: cast trunc s_12_30 -> u8
        let s_12_31: bool = ((s_12_30) != 0);
        // D s_12_32: write-var uxn_table <= s_12_31
        fn_state.uxn_table = s_12_31;
        // C s_12_33: const #59s : i
        let s_12_33: i128 = 59;
        // D s_12_34: read-var descriptor:bv
        let s_12_34: Bits = fn_state.descriptor;
        // C s_12_35: const #1u : u64
        let s_12_35: u64 = 1;
        // D s_12_36: bit-extract s_12_34 s_12_33 s_12_35
        let s_12_36: Bits = (Bits::new(
            ((s_12_34) >> (s_12_33)).value(),
            u16::try_from(s_12_35).unwrap(),
        ));
        // D s_12_37: cast reint s_12_36 -> u8
        let s_12_37: bool = ((s_12_36.value()) != 0);
        // C s_12_38: const #0s : i
        let s_12_38: i128 = 0;
        // C s_12_39: const #0u : u64
        let s_12_39: u64 = 0;
        // D s_12_40: cast zx s_12_37 -> u64
        let s_12_40: u64 = (s_12_37 as u64);
        // C s_12_41: const #1u : u64
        let s_12_41: u64 = 1;
        // D s_12_42: and s_12_40 s_12_41
        let s_12_42: u64 = ((s_12_40) & (s_12_41));
        // D s_12_43: cmp-eq s_12_42 s_12_41
        let s_12_43: bool = ((s_12_42) == (s_12_41));
        // D s_12_44: lsl s_12_40 s_12_38
        let s_12_44: u64 = s_12_40 << s_12_38;
        // D s_12_45: or s_12_39 s_12_44
        let s_12_45: u64 = ((s_12_39) | (s_12_44));
        // D s_12_46: cmpl s_12_44
        let s_12_46: u64 = !s_12_44;
        // D s_12_47: and s_12_39 s_12_46
        let s_12_47: u64 = ((s_12_39) & (s_12_46));
        // D s_12_48: select s_12_43 s_12_45 s_12_47
        let s_12_48: u64 = if s_12_43 { s_12_45 } else { s_12_47 };
        // D s_12_49: cast trunc s_12_48 -> u8
        let s_12_49: bool = ((s_12_48) != 0);
        // D s_12_50: write-var pxn_table <= s_12_49
        fn_state.pxn_table = s_12_49;
        // N s_12_51: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_13_0: read-var permissions.1:struct
        let s_13_0: u8 = fn_state.permissions._1;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // D s_13_2: read-var ap_table:u8
        let s_13_2: u8 = fn_state.ap_table;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: or s_13_1 s_13_3
        let s_13_4: Bits = ((s_13_1) | (s_13_3));
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: u8 = (s_13_4.value() as u8);
        // D s_13_6: write-var permissions.1 <= s_13_5
        fn_state.permissions._1 = s_13_5;
        // D s_13_7: read-var permissions.16:struct
        let s_13_7: bool = fn_state.permissions._16;
        // D s_13_8: cast zx s_13_7 -> bv
        let s_13_8: Bits = Bits::new(s_13_7 as u128, 1u16);
        // D s_13_9: read-var uxn_table:u8
        let s_13_9: bool = fn_state.uxn_table;
        // D s_13_10: cast zx s_13_9 -> bv
        let s_13_10: Bits = Bits::new(s_13_9 as u128, 1u16);
        // D s_13_11: or s_13_8 s_13_10
        let s_13_11: Bits = ((s_13_8) | (s_13_10));
        // D s_13_12: cast reint s_13_11 -> u8
        let s_13_12: bool = ((s_13_11.value()) != 0);
        // D s_13_13: write-var permissions.16 <= s_13_12
        fn_state.permissions._16 = s_13_12;
        // D s_13_14: read-var permissions.6:struct
        let s_13_14: bool = fn_state.permissions._6;
        // D s_13_15: cast zx s_13_14 -> bv
        let s_13_15: Bits = Bits::new(s_13_14 as u128, 1u16);
        // D s_13_16: read-var pxn_table:u8
        let s_13_16: bool = fn_state.pxn_table;
        // D s_13_17: cast zx s_13_16 -> bv
        let s_13_17: Bits = Bits::new(s_13_16 as u128, 1u16);
        // D s_13_18: or s_13_15 s_13_17
        let s_13_18: Bits = ((s_13_15) | (s_13_17));
        // D s_13_19: cast reint s_13_18 -> u8
        let s_13_19: bool = ((s_13_18.value()) != 0);
        // D s_13_20: write-var permissions.6 <= s_13_19
        fn_state.permissions._6 = s_13_19;
        // N s_13_21: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_14_0: read-var descriptor:bv
        let s_14_0: Bits = fn_state.descriptor;
        // D s_14_1: size-of s_14_0
        let s_14_1: u16 = s_14_0.length();
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // C s_14_3: const #126s : i
        let s_14_3: i128 = 126;
        // D s_14_4: cmp-lt s_14_3 s_14_2
        let s_14_4: bool = ((s_14_3) < (s_14_2));
        // N s_14_5: assert s_14_4
        let s_14_5: () = assert!(s_14_4);
        // C s_14_6: const #125s : i
        let s_14_6: i128 = 125;
        // D s_14_7: read-var descriptor:bv
        let s_14_7: Bits = fn_state.descriptor;
        // C s_14_8: const #1s : i64
        let s_14_8: i64 = 1;
        // C s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // C s_14_10: const #1s : i
        let s_14_10: i128 = 1;
        // C s_14_11: add s_14_10 s_14_9
        let s_14_11: i128 = (s_14_10 + s_14_9);
        // D s_14_12: bit-extract s_14_7 s_14_6 s_14_11
        let s_14_12: Bits = (Bits::new(
            ((s_14_7) >> (s_14_6)).value(),
            u16::try_from(s_14_11).unwrap(),
        ));
        // D s_14_13: cast reint s_14_12 -> u8
        let s_14_13: u8 = (s_14_12.value() as u8);
        // D s_14_14: write-var ap_table <= s_14_13
        fn_state.ap_table = s_14_13;
        // C s_14_15: const #124s : i
        let s_14_15: i128 = 124;
        // D s_14_16: read-var descriptor:bv
        let s_14_16: Bits = fn_state.descriptor;
        // C s_14_17: const #1u : u64
        let s_14_17: u64 = 1;
        // D s_14_18: bit-extract s_14_16 s_14_15 s_14_17
        let s_14_18: Bits = (Bits::new(
            ((s_14_16) >> (s_14_15)).value(),
            u16::try_from(s_14_17).unwrap(),
        ));
        // D s_14_19: cast reint s_14_18 -> u8
        let s_14_19: bool = ((s_14_18.value()) != 0);
        // C s_14_20: const #0s : i
        let s_14_20: i128 = 0;
        // C s_14_21: const #0u : u64
        let s_14_21: u64 = 0;
        // D s_14_22: cast zx s_14_19 -> u64
        let s_14_22: u64 = (s_14_19 as u64);
        // C s_14_23: const #1u : u64
        let s_14_23: u64 = 1;
        // D s_14_24: and s_14_22 s_14_23
        let s_14_24: u64 = ((s_14_22) & (s_14_23));
        // D s_14_25: cmp-eq s_14_24 s_14_23
        let s_14_25: bool = ((s_14_24) == (s_14_23));
        // D s_14_26: lsl s_14_22 s_14_20
        let s_14_26: u64 = s_14_22 << s_14_20;
        // D s_14_27: or s_14_21 s_14_26
        let s_14_27: u64 = ((s_14_21) | (s_14_26));
        // D s_14_28: cmpl s_14_26
        let s_14_28: u64 = !s_14_26;
        // D s_14_29: and s_14_21 s_14_28
        let s_14_29: u64 = ((s_14_21) & (s_14_28));
        // D s_14_30: select s_14_25 s_14_27 s_14_29
        let s_14_30: u64 = if s_14_25 { s_14_27 } else { s_14_29 };
        // D s_14_31: cast trunc s_14_30 -> u8
        let s_14_31: bool = ((s_14_30) != 0);
        // D s_14_32: write-var uxn_table <= s_14_31
        fn_state.uxn_table = s_14_31;
        // C s_14_33: const #123s : i
        let s_14_33: i128 = 123;
        // D s_14_34: read-var descriptor:bv
        let s_14_34: Bits = fn_state.descriptor;
        // C s_14_35: const #1u : u64
        let s_14_35: u64 = 1;
        // D s_14_36: bit-extract s_14_34 s_14_33 s_14_35
        let s_14_36: Bits = (Bits::new(
            ((s_14_34) >> (s_14_33)).value(),
            u16::try_from(s_14_35).unwrap(),
        ));
        // D s_14_37: cast reint s_14_36 -> u8
        let s_14_37: bool = ((s_14_36.value()) != 0);
        // C s_14_38: const #0s : i
        let s_14_38: i128 = 0;
        // C s_14_39: const #0u : u64
        let s_14_39: u64 = 0;
        // D s_14_40: cast zx s_14_37 -> u64
        let s_14_40: u64 = (s_14_37 as u64);
        // C s_14_41: const #1u : u64
        let s_14_41: u64 = 1;
        // D s_14_42: and s_14_40 s_14_41
        let s_14_42: u64 = ((s_14_40) & (s_14_41));
        // D s_14_43: cmp-eq s_14_42 s_14_41
        let s_14_43: bool = ((s_14_42) == (s_14_41));
        // D s_14_44: lsl s_14_40 s_14_38
        let s_14_44: u64 = s_14_40 << s_14_38;
        // D s_14_45: or s_14_39 s_14_44
        let s_14_45: u64 = ((s_14_39) | (s_14_44));
        // D s_14_46: cmpl s_14_44
        let s_14_46: u64 = !s_14_44;
        // D s_14_47: and s_14_39 s_14_46
        let s_14_47: u64 = ((s_14_39) & (s_14_46));
        // D s_14_48: select s_14_43 s_14_45 s_14_47
        let s_14_48: u64 = if s_14_43 { s_14_45 } else { s_14_47 };
        // D s_14_49: cast trunc s_14_48 -> u8
        let s_14_49: bool = ((s_14_48) != 0);
        // D s_14_50: write-var pxn_table <= s_14_49
        fn_state.pxn_table = s_14_49;
        // N s_14_51: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_15_0: read-var walkparams.3:struct
        let s_15_0: bool = fn_state.walkparams._3;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // N s_15_5: branch s_15_4 b18 b16
        if s_15_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_16_0: read-var descriptor:bv
        let s_16_0: Bits = fn_state.descriptor;
        // D s_16_1: size-of s_16_0
        let s_16_1: u16 = s_16_0.length();
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // C s_16_3: const #62s : i
        let s_16_3: i128 = 62;
        // D s_16_4: cmp-lt s_16_3 s_16_2
        let s_16_4: bool = ((s_16_3) < (s_16_2));
        // N s_16_5: assert s_16_4
        let s_16_5: () = assert!(s_16_4);
        // C s_16_6: const #62s : i
        let s_16_6: i128 = 62;
        // D s_16_7: read-var descriptor:bv
        let s_16_7: Bits = fn_state.descriptor;
        // C s_16_8: const #1u : u64
        let s_16_8: u64 = 1;
        // D s_16_9: bit-extract s_16_7 s_16_6 s_16_8
        let s_16_9: Bits = (Bits::new(
            ((s_16_7) >> (s_16_6)).value(),
            u16::try_from(s_16_8).unwrap(),
        ));
        // D s_16_10: cast reint s_16_9 -> u8
        let s_16_10: bool = ((s_16_9.value()) != 0);
        // C s_16_11: const #0s : i
        let s_16_11: i128 = 0;
        // C s_16_12: const #0u : u64
        let s_16_12: u64 = 0;
        // D s_16_13: cast zx s_16_10 -> u64
        let s_16_13: u64 = (s_16_10 as u64);
        // C s_16_14: const #1u : u64
        let s_16_14: u64 = 1;
        // D s_16_15: and s_16_13 s_16_14
        let s_16_15: u64 = ((s_16_13) & (s_16_14));
        // D s_16_16: cmp-eq s_16_15 s_16_14
        let s_16_16: bool = ((s_16_15) == (s_16_14));
        // D s_16_17: lsl s_16_13 s_16_11
        let s_16_17: u64 = s_16_13 << s_16_11;
        // D s_16_18: or s_16_12 s_16_17
        let s_16_18: u64 = ((s_16_12) | (s_16_17));
        // D s_16_19: cmpl s_16_17
        let s_16_19: u64 = !s_16_17;
        // D s_16_20: and s_16_12 s_16_19
        let s_16_20: u64 = ((s_16_12) & (s_16_19));
        // D s_16_21: select s_16_16 s_16_18 s_16_20
        let s_16_21: u64 = if s_16_16 { s_16_18 } else { s_16_20 };
        // D s_16_22: cast trunc s_16_21 -> u8
        let s_16_22: bool = ((s_16_21) != 0);
        // D s_16_23: cast zx s_16_22 -> bv
        let s_16_23: Bits = Bits::new(s_16_22 as u128, 1u16);
        // C s_16_24: const #0u : u8
        let s_16_24: bool = false;
        // C s_16_25: cast zx s_16_24 -> bv
        let s_16_25: Bits = Bits::new(s_16_24 as u128, 1u16);
        // D s_16_26: cast reint s_16_23 -> u128
        let s_16_26: u128 = (s_16_23.value() as u128);
        // D s_16_27: size-of s_16_23
        let s_16_27: u16 = s_16_23.length();
        // C s_16_28: cast reint s_16_25 -> u128
        let s_16_28: u128 = (s_16_25.value() as u128);
        // D s_16_29: size-of s_16_25
        let s_16_29: u16 = s_16_25.length();
        // D s_16_30: lsl s_16_26 s_16_29
        let s_16_30: u128 = s_16_26 << s_16_29;
        // D s_16_31: or s_16_30 s_16_28
        let s_16_31: u128 = ((s_16_30) | (s_16_28));
        // D s_16_32: add s_16_27 s_16_29
        let s_16_32: u16 = (s_16_27 + s_16_29);
        // D s_16_33: create-bits s_16_31 s_16_32
        let s_16_33: Bits = Bits::new(s_16_31, s_16_32);
        // D s_16_34: cast reint s_16_33 -> u8
        let s_16_34: u8 = (s_16_33.value() as u8);
        // D s_16_35: write-var ap_table <= s_16_34
        fn_state.ap_table = s_16_34;
        // C s_16_36: const #60s : i
        let s_16_36: i128 = 60;
        // D s_16_37: read-var descriptor:bv
        let s_16_37: Bits = fn_state.descriptor;
        // C s_16_38: const #1u : u64
        let s_16_38: u64 = 1;
        // D s_16_39: bit-extract s_16_37 s_16_36 s_16_38
        let s_16_39: Bits = (Bits::new(
            ((s_16_37) >> (s_16_36)).value(),
            u16::try_from(s_16_38).unwrap(),
        ));
        // D s_16_40: cast reint s_16_39 -> u8
        let s_16_40: bool = ((s_16_39.value()) != 0);
        // C s_16_41: const #0s : i
        let s_16_41: i128 = 0;
        // C s_16_42: const #0u : u64
        let s_16_42: u64 = 0;
        // D s_16_43: cast zx s_16_40 -> u64
        let s_16_43: u64 = (s_16_40 as u64);
        // C s_16_44: const #1u : u64
        let s_16_44: u64 = 1;
        // D s_16_45: and s_16_43 s_16_44
        let s_16_45: u64 = ((s_16_43) & (s_16_44));
        // D s_16_46: cmp-eq s_16_45 s_16_44
        let s_16_46: bool = ((s_16_45) == (s_16_44));
        // D s_16_47: lsl s_16_43 s_16_41
        let s_16_47: u64 = s_16_43 << s_16_41;
        // D s_16_48: or s_16_42 s_16_47
        let s_16_48: u64 = ((s_16_42) | (s_16_47));
        // D s_16_49: cmpl s_16_47
        let s_16_49: u64 = !s_16_47;
        // D s_16_50: and s_16_42 s_16_49
        let s_16_50: u64 = ((s_16_42) & (s_16_49));
        // D s_16_51: select s_16_46 s_16_48 s_16_50
        let s_16_51: u64 = if s_16_46 { s_16_48 } else { s_16_50 };
        // D s_16_52: cast trunc s_16_51 -> u8
        let s_16_52: bool = ((s_16_51) != 0);
        // D s_16_53: write-var pxn_table <= s_16_52
        fn_state.pxn_table = s_16_52;
        // N s_16_54: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_17_0: read-var permissions.1:struct
        let s_17_0: u8 = fn_state.permissions._1;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // D s_17_2: read-var ap_table:u8
        let s_17_2: u8 = fn_state.ap_table;
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: or s_17_1 s_17_3
        let s_17_4: Bits = ((s_17_1) | (s_17_3));
        // D s_17_5: cast reint s_17_4 -> u8
        let s_17_5: u8 = (s_17_4.value() as u8);
        // D s_17_6: write-var permissions.1 <= s_17_5
        fn_state.permissions._1 = s_17_5;
        // D s_17_7: read-var permissions.6:struct
        let s_17_7: bool = fn_state.permissions._6;
        // D s_17_8: cast zx s_17_7 -> bv
        let s_17_8: Bits = Bits::new(s_17_7 as u128, 1u16);
        // D s_17_9: read-var pxn_table:u8
        let s_17_9: bool = fn_state.pxn_table;
        // D s_17_10: cast zx s_17_9 -> bv
        let s_17_10: Bits = Bits::new(s_17_9 as u128, 1u16);
        // D s_17_11: or s_17_8 s_17_10
        let s_17_11: Bits = ((s_17_8) | (s_17_10));
        // D s_17_12: cast reint s_17_11 -> u8
        let s_17_12: bool = ((s_17_11.value()) != 0);
        // D s_17_13: write-var permissions.6 <= s_17_12
        fn_state.permissions._6 = s_17_12;
        // N s_17_14: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_18_0: read-var descriptor:bv
        let s_18_0: Bits = fn_state.descriptor;
        // D s_18_1: size-of s_18_0
        let s_18_1: u16 = s_18_0.length();
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // C s_18_3: const #126s : i
        let s_18_3: i128 = 126;
        // D s_18_4: cmp-lt s_18_3 s_18_2
        let s_18_4: bool = ((s_18_3) < (s_18_2));
        // N s_18_5: assert s_18_4
        let s_18_5: () = assert!(s_18_4);
        // C s_18_6: const #126s : i
        let s_18_6: i128 = 126;
        // D s_18_7: read-var descriptor:bv
        let s_18_7: Bits = fn_state.descriptor;
        // C s_18_8: const #1u : u64
        let s_18_8: u64 = 1;
        // D s_18_9: bit-extract s_18_7 s_18_6 s_18_8
        let s_18_9: Bits = (Bits::new(
            ((s_18_7) >> (s_18_6)).value(),
            u16::try_from(s_18_8).unwrap(),
        ));
        // D s_18_10: cast reint s_18_9 -> u8
        let s_18_10: bool = ((s_18_9.value()) != 0);
        // C s_18_11: const #0s : i
        let s_18_11: i128 = 0;
        // C s_18_12: const #0u : u64
        let s_18_12: u64 = 0;
        // D s_18_13: cast zx s_18_10 -> u64
        let s_18_13: u64 = (s_18_10 as u64);
        // C s_18_14: const #1u : u64
        let s_18_14: u64 = 1;
        // D s_18_15: and s_18_13 s_18_14
        let s_18_15: u64 = ((s_18_13) & (s_18_14));
        // D s_18_16: cmp-eq s_18_15 s_18_14
        let s_18_16: bool = ((s_18_15) == (s_18_14));
        // D s_18_17: lsl s_18_13 s_18_11
        let s_18_17: u64 = s_18_13 << s_18_11;
        // D s_18_18: or s_18_12 s_18_17
        let s_18_18: u64 = ((s_18_12) | (s_18_17));
        // D s_18_19: cmpl s_18_17
        let s_18_19: u64 = !s_18_17;
        // D s_18_20: and s_18_12 s_18_19
        let s_18_20: u64 = ((s_18_12) & (s_18_19));
        // D s_18_21: select s_18_16 s_18_18 s_18_20
        let s_18_21: u64 = if s_18_16 { s_18_18 } else { s_18_20 };
        // D s_18_22: cast trunc s_18_21 -> u8
        let s_18_22: bool = ((s_18_21) != 0);
        // D s_18_23: cast zx s_18_22 -> bv
        let s_18_23: Bits = Bits::new(s_18_22 as u128, 1u16);
        // C s_18_24: const #0u : u8
        let s_18_24: bool = false;
        // C s_18_25: cast zx s_18_24 -> bv
        let s_18_25: Bits = Bits::new(s_18_24 as u128, 1u16);
        // D s_18_26: cast reint s_18_23 -> u128
        let s_18_26: u128 = (s_18_23.value() as u128);
        // D s_18_27: size-of s_18_23
        let s_18_27: u16 = s_18_23.length();
        // C s_18_28: cast reint s_18_25 -> u128
        let s_18_28: u128 = (s_18_25.value() as u128);
        // D s_18_29: size-of s_18_25
        let s_18_29: u16 = s_18_25.length();
        // D s_18_30: lsl s_18_26 s_18_29
        let s_18_30: u128 = s_18_26 << s_18_29;
        // D s_18_31: or s_18_30 s_18_28
        let s_18_31: u128 = ((s_18_30) | (s_18_28));
        // D s_18_32: add s_18_27 s_18_29
        let s_18_32: u16 = (s_18_27 + s_18_29);
        // D s_18_33: create-bits s_18_31 s_18_32
        let s_18_33: Bits = Bits::new(s_18_31, s_18_32);
        // D s_18_34: cast reint s_18_33 -> u8
        let s_18_34: u8 = (s_18_33.value() as u8);
        // D s_18_35: write-var ap_table <= s_18_34
        fn_state.ap_table = s_18_34;
        // C s_18_36: const #124s : i
        let s_18_36: i128 = 124;
        // D s_18_37: read-var descriptor:bv
        let s_18_37: Bits = fn_state.descriptor;
        // C s_18_38: const #1u : u64
        let s_18_38: u64 = 1;
        // D s_18_39: bit-extract s_18_37 s_18_36 s_18_38
        let s_18_39: Bits = (Bits::new(
            ((s_18_37) >> (s_18_36)).value(),
            u16::try_from(s_18_38).unwrap(),
        ));
        // D s_18_40: cast reint s_18_39 -> u8
        let s_18_40: bool = ((s_18_39.value()) != 0);
        // C s_18_41: const #0s : i
        let s_18_41: i128 = 0;
        // C s_18_42: const #0u : u64
        let s_18_42: u64 = 0;
        // D s_18_43: cast zx s_18_40 -> u64
        let s_18_43: u64 = (s_18_40 as u64);
        // C s_18_44: const #1u : u64
        let s_18_44: u64 = 1;
        // D s_18_45: and s_18_43 s_18_44
        let s_18_45: u64 = ((s_18_43) & (s_18_44));
        // D s_18_46: cmp-eq s_18_45 s_18_44
        let s_18_46: bool = ((s_18_45) == (s_18_44));
        // D s_18_47: lsl s_18_43 s_18_41
        let s_18_47: u64 = s_18_43 << s_18_41;
        // D s_18_48: or s_18_42 s_18_47
        let s_18_48: u64 = ((s_18_42) | (s_18_47));
        // D s_18_49: cmpl s_18_47
        let s_18_49: u64 = !s_18_47;
        // D s_18_50: and s_18_42 s_18_49
        let s_18_50: u64 = ((s_18_42) & (s_18_49));
        // D s_18_51: select s_18_46 s_18_48 s_18_50
        let s_18_51: u64 = if s_18_46 { s_18_48 } else { s_18_50 };
        // D s_18_52: cast trunc s_18_51 -> u8
        let s_18_52: bool = ((s_18_51) != 0);
        // D s_18_53: write-var pxn_table <= s_18_52
        fn_state.pxn_table = s_18_52;
        // N s_18_54: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_19_0: read-var walkparams.22:struct
        let s_19_0: bool = fn_state.walkparams._22;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #1u : u8
        let s_19_2: bool = true;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var gs#18128 <= s_19_4
        fn_state.gs_18128 = s_19_4;
        // N s_19_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call EL2Enabled(s_20_0)
        let s_20_1: bool = EL2Enabled(state, tracer, s_20_0);
        // D s_20_2: write-var gs#18127 <= s_20_1
        fn_state.gs_18127 = s_20_1;
        // N s_20_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}

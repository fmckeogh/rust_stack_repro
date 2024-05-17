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
use AArch64_S1E0POEnabled::*;
use Bit::*;
use EL2Enabled::*;
use AArch64_S1POEnabled::*;
use HasUnprivileged::*;
use common::*;
pub fn AArch64_S1ApplyOutputPerms<T: Tracer>(
    state: &mut State,
    tracer: &T,
    permissions_in: ProductTypebf05c51f33174538,
    descriptor: Bits,
    regime: u32,
    walkparams: ProductTypeef284266e139aee2,
) -> ProductTypebf05c51f33174538 {
    #[derive(Default)]
    struct FunctionState {
        pi_index: u8,
        ga_13391: ProductType5c790c8ef59cc8b2,
        gs_17885: bool,
        gs_17959: bool,
        permissions: ProductTypebf05c51f33174538,
        e0poe: bool,
        gs_17887: bool,
        gs_17890: bool,
        gs_17884: bool,
        gs_17960: bool,
        ga_13389: ProductType5c790c8ef59cc8b2,
        poe: bool,
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
        // D s_0_2: read-var walkparams.24:struct
        let s_0_2: bool = fn_state.walkparams._24;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // C s_0_4: const #1u : u8
        let s_0_4: bool = true;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 1u16);
        // D s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: branch s_0_6 b34 b1
        if s_0_6 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_1_0: read-var regime:u32
        let s_1_0: u32 = fn_state.regime;
        // C s_1_1: const #4u : u32
        let s_1_1: u32 = 4;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b33 b2
        if s_1_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#17884 <= s_2_0
        fn_state.gs_17884 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_3_0: read-var gs#17884:u8
        let s_3_0: bool = fn_state.gs_17884;
        // N s_3_1: branch s_3_0 b32 b4
        if s_3_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#17885 <= s_4_0
        fn_state.gs_17885 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_5_0: read-var gs#17885:u8
        let s_5_0: bool = fn_state.gs_17885;
        // N s_5_1: branch s_5_0 b31 b6
        if s_5_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_6_0: read-var regime:u32
        let s_6_0: u32 = fn_state.regime;
        // D s_6_1: call HasUnprivileged(s_6_0)
        let s_6_1: bool = HasUnprivileged(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b30 b7
        if s_6_1 {
            return block_30(state, tracer, fn_state);
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
        // C s_7_3: const #7s : i
        let s_7_3: i128 = 7;
        // D s_7_4: cmp-lt s_7_3 s_7_2
        let s_7_4: bool = ((s_7_3) < (s_7_2));
        // N s_7_5: assert s_7_4
        let s_7_5: () = assert!(s_7_4);
        // D s_7_6: read-var permissions:struct
        let s_7_6: ProductTypebf05c51f33174538 = fn_state.permissions;
        // D s_7_7: write-var permissions <= s_7_6
        fn_state.permissions = s_7_6;
        // D s_7_8: read-var descriptor:bv
        let s_7_8: Bits = fn_state.descriptor;
        // D s_7_9: size-of s_7_8
        let s_7_9: u16 = s_7_8.length();
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // C s_7_11: const #54s : i
        let s_7_11: i128 = 54;
        // D s_7_12: cmp-lt s_7_11 s_7_10
        let s_7_12: bool = ((s_7_11) < (s_7_10));
        // N s_7_13: assert s_7_12
        let s_7_13: () = assert!(s_7_12);
        // C s_7_14: const #54s : i
        let s_7_14: i128 = 54;
        // D s_7_15: read-var descriptor:bv
        let s_7_15: Bits = fn_state.descriptor;
        // C s_7_16: const #1u : u64
        let s_7_16: u64 = 1;
        // D s_7_17: bit-extract s_7_15 s_7_14 s_7_16
        let s_7_17: Bits = (Bits::new(
            ((s_7_15) >> (s_7_14)).value(),
            u16::try_from(s_7_16).unwrap(),
        ));
        // D s_7_18: cast reint s_7_17 -> u8
        let s_7_18: bool = ((s_7_17.value()) != 0);
        // C s_7_19: const #0s : i
        let s_7_19: i128 = 0;
        // C s_7_20: const #0u : u64
        let s_7_20: u64 = 0;
        // D s_7_21: cast zx s_7_18 -> u64
        let s_7_21: u64 = (s_7_18 as u64);
        // C s_7_22: const #1u : u64
        let s_7_22: u64 = 1;
        // D s_7_23: and s_7_21 s_7_22
        let s_7_23: u64 = ((s_7_21) & (s_7_22));
        // D s_7_24: cmp-eq s_7_23 s_7_22
        let s_7_24: bool = ((s_7_23) == (s_7_22));
        // D s_7_25: lsl s_7_21 s_7_19
        let s_7_25: u64 = s_7_21 << s_7_19;
        // D s_7_26: or s_7_20 s_7_25
        let s_7_26: u64 = ((s_7_20) | (s_7_25));
        // D s_7_27: cmpl s_7_25
        let s_7_27: u64 = !s_7_25;
        // D s_7_28: and s_7_20 s_7_27
        let s_7_28: u64 = ((s_7_20) & (s_7_27));
        // D s_7_29: select s_7_24 s_7_26 s_7_28
        let s_7_29: u64 = if s_7_24 { s_7_26 } else { s_7_28 };
        // D s_7_30: cast trunc s_7_29 -> u8
        let s_7_30: bool = ((s_7_29) != 0);
        // D s_7_31: write-var permissions.17 <= s_7_30
        fn_state.permissions._17 = s_7_30;
        // N s_7_32: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_8_0: read-var descriptor:bv
        let s_8_0: Bits = fn_state.descriptor;
        // D s_8_1: size-of s_8_0
        let s_8_1: u16 = s_8_0.length();
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // C s_8_3: const #51s : i
        let s_8_3: i128 = 51;
        // D s_8_4: cmp-lt s_8_3 s_8_2
        let s_8_4: bool = ((s_8_3) < (s_8_2));
        // N s_8_5: assert s_8_4
        let s_8_5: () = assert!(s_8_4);
        // D s_8_6: read-var walkparams.12:struct
        let s_8_6: bool = fn_state.walkparams._12;
        // D s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 1u16);
        // C s_8_8: const #1u : u8
        let s_8_8: bool = true;
        // C s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 1u16);
        // D s_8_10: cmp-eq s_8_7 s_8_9
        let s_8_10: bool = ((s_8_7) == (s_8_9));
        // N s_8_11: branch s_8_10 b29 b9
        if s_8_10 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#17887 <= s_9_0
        fn_state.gs_17887 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_10_0: read-var gs#17887:u8
        let s_10_0: bool = fn_state.gs_17887;
        // N s_10_1: branch s_10_0 b28 b11
        if s_10_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#17890 <= s_11_0
        fn_state.gs_17890 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_12_0: read-var gs#17890:u8
        let s_12_0: bool = fn_state.gs_17890;
        // N s_12_1: branch s_12_0 b27 b13
        if s_12_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_15_0: read-var regime:u32
        let s_15_0: u32 = fn_state.regime;
        // D s_15_1: call AArch64_S1POEnabled(s_15_0)
        let s_15_1: bool = AArch64_S1POEnabled(state, tracer, s_15_0);
        // D s_15_2: write-var poe <= s_15_1
        fn_state.poe = s_15_1;
        // D s_15_3: read-var regime:u32
        let s_15_3: u32 = fn_state.regime;
        // D s_15_4: call HasUnprivileged(s_15_3)
        let s_15_4: bool = HasUnprivileged(state, tracer, s_15_3);
        // N s_15_5: branch s_15_4 b26 b16
        if s_15_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#17959 <= s_16_0
        fn_state.gs_17959 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_17_0: read-var gs#17959:u8
        let s_17_0: bool = fn_state.gs_17959;
        // D s_17_1: write-var e0poe <= s_17_0
        fn_state.e0poe = s_17_0;
        // D s_17_2: read-var poe:u8
        let s_17_2: bool = fn_state.poe;
        // N s_17_3: branch s_17_2 b25 b18
        if s_17_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_18_0: read-var e0poe:u8
        let s_18_0: bool = fn_state.e0poe;
        // D s_18_1: write-var gs#17960 <= s_18_0
        fn_state.gs_17960 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_19_0: read-var gs#17960:u8
        let s_19_0: bool = fn_state.gs_17960;
        // N s_19_1: branch s_19_0 b22 b20
        if s_19_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_21_0: read-var permissions:struct
        let s_21_0: ProductTypebf05c51f33174538 = fn_state.permissions;
        // N s_21_1: return s_21_0
        return s_21_0;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_22_0: read-var walkparams.3:struct
        let s_22_0: bool = fn_state.walkparams._3;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // N s_22_5: branch s_22_4 b24 b23
        if s_22_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_23_0: read-var descriptor:bv
        let s_23_0: Bits = fn_state.descriptor;
        // D s_23_1: size-of s_23_0
        let s_23_1: u16 = s_23_0.length();
        // D s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (i128::try_from(s_23_1).unwrap());
        // C s_23_3: const #62s : i
        let s_23_3: i128 = 62;
        // D s_23_4: cmp-lt s_23_3 s_23_2
        let s_23_4: bool = ((s_23_3) < (s_23_2));
        // N s_23_5: assert s_23_4
        let s_23_5: () = assert!(s_23_4);
        // C s_23_6: const #60s : i
        let s_23_6: i128 = 60;
        // D s_23_7: read-var descriptor:bv
        let s_23_7: Bits = fn_state.descriptor;
        // C s_23_8: const #1s : i64
        let s_23_8: i64 = 1;
        // C s_23_9: cast zx s_23_8 -> i
        let s_23_9: i128 = (i128::try_from(s_23_8).unwrap());
        // C s_23_10: const #2s : i
        let s_23_10: i128 = 2;
        // C s_23_11: add s_23_10 s_23_9
        let s_23_11: i128 = (s_23_10 + s_23_9);
        // D s_23_12: bit-extract s_23_7 s_23_6 s_23_11
        let s_23_12: Bits = (Bits::new(
            ((s_23_7) >> (s_23_6)).value(),
            u16::try_from(s_23_11).unwrap(),
        ));
        // D s_23_13: cast reint s_23_12 -> u8
        let s_23_13: u8 = (s_23_12.value() as u8);
        // C s_23_14: const #0u : u8
        let s_23_14: bool = false;
        // C s_23_15: cast zx s_23_14 -> bv
        let s_23_15: Bits = Bits::new(s_23_14 as u128, 1u16);
        // D s_23_16: cast zx s_23_13 -> bv
        let s_23_16: Bits = Bits::new(s_23_13 as u128, 3u16);
        // C s_23_17: cast reint s_23_15 -> u128
        let s_23_17: u128 = (s_23_15.value() as u128);
        // D s_23_18: size-of s_23_15
        let s_23_18: u16 = s_23_15.length();
        // D s_23_19: cast reint s_23_16 -> u128
        let s_23_19: u128 = (s_23_16.value() as u128);
        // D s_23_20: size-of s_23_16
        let s_23_20: u16 = s_23_16.length();
        // D s_23_21: lsl s_23_17 s_23_20
        let s_23_21: u128 = s_23_17 << s_23_20;
        // D s_23_22: or s_23_21 s_23_19
        let s_23_22: u128 = ((s_23_21) | (s_23_19));
        // D s_23_23: add s_23_18 s_23_20
        let s_23_23: u16 = (s_23_18 + s_23_20);
        // D s_23_24: create-bits s_23_22 s_23_23
        let s_23_24: Bits = Bits::new(s_23_22, s_23_23);
        // D s_23_25: cast reint s_23_24 -> u8
        let s_23_25: u8 = (s_23_24.value() as u8);
        // D s_23_26: write-var permissions.3 <= s_23_25
        fn_state.permissions._3 = s_23_25;
        // N s_23_27: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_24_0: read-var descriptor:bv
        let s_24_0: Bits = fn_state.descriptor;
        // D s_24_1: size-of s_24_0
        let s_24_1: u16 = s_24_0.length();
        // D s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (i128::try_from(s_24_1).unwrap());
        // C s_24_3: const #124s : i
        let s_24_3: i128 = 124;
        // D s_24_4: cmp-lt s_24_3 s_24_2
        let s_24_4: bool = ((s_24_3) < (s_24_2));
        // N s_24_5: assert s_24_4
        let s_24_5: () = assert!(s_24_4);
        // C s_24_6: const #121s : i
        let s_24_6: i128 = 121;
        // D s_24_7: read-var descriptor:bv
        let s_24_7: Bits = fn_state.descriptor;
        // C s_24_8: const #1s : i64
        let s_24_8: i64 = 1;
        // C s_24_9: cast zx s_24_8 -> i
        let s_24_9: i128 = (i128::try_from(s_24_8).unwrap());
        // C s_24_10: const #3s : i
        let s_24_10: i128 = 3;
        // C s_24_11: add s_24_10 s_24_9
        let s_24_11: i128 = (s_24_10 + s_24_9);
        // D s_24_12: bit-extract s_24_7 s_24_6 s_24_11
        let s_24_12: Bits = (Bits::new(
            ((s_24_7) >> (s_24_6)).value(),
            u16::try_from(s_24_11).unwrap(),
        ));
        // D s_24_13: cast reint s_24_12 -> u8
        let s_24_13: u8 = (s_24_12.value() as u8);
        // D s_24_14: write-var permissions.3 <= s_24_13
        fn_state.permissions._3 = s_24_13;
        // N s_24_15: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#17960 <= s_25_0
        fn_state.gs_17960 = s_25_0;
        // N s_25_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_26_0: read-var walkparams.22:struct
        let s_26_0: bool = fn_state.walkparams._22;
        // D s_26_1: read-var regime:u32
        let s_26_1: u32 = fn_state.regime;
        // D s_26_2: call AArch64_S1E0POEnabled(s_26_1, s_26_0)
        let s_26_2: bool = AArch64_S1E0POEnabled(state, tracer, s_26_1, s_26_0);
        // D s_26_3: write-var gs#17959 <= s_26_2
        fn_state.gs_17959 = s_26_2;
        // N s_26_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // S s_27_1: call Bit(s_27_0)
        let s_27_1: bool = Bit(state, tracer, s_27_0);
        // D s_27_2: read-var permissions:struct
        let s_27_2: ProductTypebf05c51f33174538 = fn_state.permissions;
        // D s_27_3: write-var permissions <= s_27_2
        fn_state.permissions = s_27_2;
        // N s_27_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_28_0: const #51s : i
        let s_28_0: i128 = 51;
        // D s_28_1: read-var descriptor:bv
        let s_28_1: Bits = fn_state.descriptor;
        // C s_28_2: const #1u : u64
        let s_28_2: u64 = 1;
        // D s_28_3: bit-extract s_28_1 s_28_0 s_28_2
        let s_28_3: Bits = (Bits::new(
            ((s_28_1) >> (s_28_0)).value(),
            u16::try_from(s_28_2).unwrap(),
        ));
        // D s_28_4: cast reint s_28_3 -> u8
        let s_28_4: bool = ((s_28_3.value()) != 0);
        // C s_28_5: const #0s : i
        let s_28_5: i128 = 0;
        // C s_28_6: const #0u : u64
        let s_28_6: u64 = 0;
        // D s_28_7: cast zx s_28_4 -> u64
        let s_28_7: u64 = (s_28_4 as u64);
        // C s_28_8: const #1u : u64
        let s_28_8: u64 = 1;
        // D s_28_9: and s_28_7 s_28_8
        let s_28_9: u64 = ((s_28_7) & (s_28_8));
        // D s_28_10: cmp-eq s_28_9 s_28_8
        let s_28_10: bool = ((s_28_9) == (s_28_8));
        // D s_28_11: lsl s_28_7 s_28_5
        let s_28_11: u64 = s_28_7 << s_28_5;
        // D s_28_12: or s_28_6 s_28_11
        let s_28_12: u64 = ((s_28_6) | (s_28_11));
        // D s_28_13: cmpl s_28_11
        let s_28_13: u64 = !s_28_11;
        // D s_28_14: and s_28_6 s_28_13
        let s_28_14: u64 = ((s_28_6) & (s_28_13));
        // D s_28_15: select s_28_10 s_28_12 s_28_14
        let s_28_15: u64 = if s_28_10 { s_28_12 } else { s_28_14 };
        // D s_28_16: cast trunc s_28_15 -> u8
        let s_28_16: bool = ((s_28_15) != 0);
        // D s_28_17: cast zx s_28_16 -> bv
        let s_28_17: Bits = Bits::new(s_28_16 as u128, 1u16);
        // C s_28_18: const #1u : u8
        let s_28_18: bool = true;
        // C s_28_19: cast zx s_28_18 -> bv
        let s_28_19: Bits = Bits::new(s_28_18 as u128, 1u16);
        // D s_28_20: cmp-eq s_28_17 s_28_19
        let s_28_20: bool = ((s_28_17) == (s_28_19));
        // D s_28_21: write-var gs#17890 <= s_28_20
        fn_state.gs_17890 = s_28_20;
        // N s_28_22: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_29_0: read-var walkparams.14:struct
        let s_29_0: bool = fn_state.walkparams._14;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#17887 <= s_29_4
        fn_state.gs_17887 = s_29_4;
        // N s_29_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_30_0: read-var descriptor:bv
        let s_30_0: Bits = fn_state.descriptor;
        // D s_30_1: size-of s_30_0
        let s_30_1: u16 = s_30_0.length();
        // D s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (i128::try_from(s_30_1).unwrap());
        // C s_30_3: const #7s : i
        let s_30_3: i128 = 7;
        // D s_30_4: cmp-lt s_30_3 s_30_2
        let s_30_4: bool = ((s_30_3) < (s_30_2));
        // N s_30_5: assert s_30_4
        let s_30_5: () = assert!(s_30_4);
        // D s_30_6: read-var permissions:struct
        let s_30_6: ProductTypebf05c51f33174538 = fn_state.permissions;
        // D s_30_7: write-var permissions <= s_30_6
        fn_state.permissions = s_30_6;
        // D s_30_8: read-var descriptor:bv
        let s_30_8: Bits = fn_state.descriptor;
        // D s_30_9: size-of s_30_8
        let s_30_9: u16 = s_30_8.length();
        // D s_30_10: cast zx s_30_9 -> i
        let s_30_10: i128 = (i128::try_from(s_30_9).unwrap());
        // C s_30_11: const #54s : i
        let s_30_11: i128 = 54;
        // D s_30_12: cmp-lt s_30_11 s_30_10
        let s_30_12: bool = ((s_30_11) < (s_30_10));
        // N s_30_13: assert s_30_12
        let s_30_13: () = assert!(s_30_12);
        // C s_30_14: const #54s : i
        let s_30_14: i128 = 54;
        // D s_30_15: read-var descriptor:bv
        let s_30_15: Bits = fn_state.descriptor;
        // C s_30_16: const #1u : u64
        let s_30_16: u64 = 1;
        // D s_30_17: bit-extract s_30_15 s_30_14 s_30_16
        let s_30_17: Bits = (Bits::new(
            ((s_30_15) >> (s_30_14)).value(),
            u16::try_from(s_30_16).unwrap(),
        ));
        // D s_30_18: cast reint s_30_17 -> u8
        let s_30_18: bool = ((s_30_17.value()) != 0);
        // C s_30_19: const #0s : i
        let s_30_19: i128 = 0;
        // C s_30_20: const #0u : u64
        let s_30_20: u64 = 0;
        // D s_30_21: cast zx s_30_18 -> u64
        let s_30_21: u64 = (s_30_18 as u64);
        // C s_30_22: const #1u : u64
        let s_30_22: u64 = 1;
        // D s_30_23: and s_30_21 s_30_22
        let s_30_23: u64 = ((s_30_21) & (s_30_22));
        // D s_30_24: cmp-eq s_30_23 s_30_22
        let s_30_24: bool = ((s_30_23) == (s_30_22));
        // D s_30_25: lsl s_30_21 s_30_19
        let s_30_25: u64 = s_30_21 << s_30_19;
        // D s_30_26: or s_30_20 s_30_25
        let s_30_26: u64 = ((s_30_20) | (s_30_25));
        // D s_30_27: cmpl s_30_25
        let s_30_27: u64 = !s_30_25;
        // D s_30_28: and s_30_20 s_30_27
        let s_30_28: u64 = ((s_30_20) & (s_30_27));
        // D s_30_29: select s_30_24 s_30_26 s_30_28
        let s_30_29: u64 = if s_30_24 { s_30_26 } else { s_30_28 };
        // D s_30_30: cast trunc s_30_29 -> u8
        let s_30_30: bool = ((s_30_29) != 0);
        // D s_30_31: write-var permissions.15 <= s_30_30
        fn_state.permissions._15 = s_30_30;
        // C s_30_32: const #53s : i
        let s_30_32: i128 = 53;
        // D s_30_33: read-var descriptor:bv
        let s_30_33: Bits = fn_state.descriptor;
        // C s_30_34: const #1u : u64
        let s_30_34: u64 = 1;
        // D s_30_35: bit-extract s_30_33 s_30_32 s_30_34
        let s_30_35: Bits = (Bits::new(
            ((s_30_33) >> (s_30_32)).value(),
            u16::try_from(s_30_34).unwrap(),
        ));
        // D s_30_36: cast reint s_30_35 -> u8
        let s_30_36: bool = ((s_30_35.value()) != 0);
        // C s_30_37: const #0s : i
        let s_30_37: i128 = 0;
        // C s_30_38: const #0u : u64
        let s_30_38: u64 = 0;
        // D s_30_39: cast zx s_30_36 -> u64
        let s_30_39: u64 = (s_30_36 as u64);
        // C s_30_40: const #1u : u64
        let s_30_40: u64 = 1;
        // D s_30_41: and s_30_39 s_30_40
        let s_30_41: u64 = ((s_30_39) & (s_30_40));
        // D s_30_42: cmp-eq s_30_41 s_30_40
        let s_30_42: bool = ((s_30_41) == (s_30_40));
        // D s_30_43: lsl s_30_39 s_30_37
        let s_30_43: u64 = s_30_39 << s_30_37;
        // D s_30_44: or s_30_38 s_30_43
        let s_30_44: u64 = ((s_30_38) | (s_30_43));
        // D s_30_45: cmpl s_30_43
        let s_30_45: u64 = !s_30_43;
        // D s_30_46: and s_30_38 s_30_45
        let s_30_46: u64 = ((s_30_38) & (s_30_45));
        // D s_30_47: select s_30_42 s_30_44 s_30_46
        let s_30_47: u64 = if s_30_42 { s_30_44 } else { s_30_46 };
        // D s_30_48: cast trunc s_30_47 -> u8
        let s_30_48: bool = ((s_30_47) != 0);
        // D s_30_49: write-var permissions.5 <= s_30_48
        fn_state.permissions._5 = s_30_48;
        // N s_30_50: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_31_0: read-var descriptor:bv
        let s_31_0: Bits = fn_state.descriptor;
        // D s_31_1: size-of s_31_0
        let s_31_1: u16 = s_31_0.length();
        // D s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (i128::try_from(s_31_1).unwrap());
        // C s_31_3: const #7s : i
        let s_31_3: i128 = 7;
        // D s_31_4: cmp-lt s_31_3 s_31_2
        let s_31_4: bool = ((s_31_3) < (s_31_2));
        // N s_31_5: assert s_31_4
        let s_31_5: () = assert!(s_31_4);
        // D s_31_6: read-var permissions:struct
        let s_31_6: ProductTypebf05c51f33174538 = fn_state.permissions;
        // D s_31_7: write-var permissions <= s_31_6
        fn_state.permissions = s_31_6;
        // D s_31_8: read-var descriptor:bv
        let s_31_8: Bits = fn_state.descriptor;
        // D s_31_9: size-of s_31_8
        let s_31_9: u16 = s_31_8.length();
        // D s_31_10: cast zx s_31_9 -> i
        let s_31_10: i128 = (i128::try_from(s_31_9).unwrap());
        // C s_31_11: const #54s : i
        let s_31_11: i128 = 54;
        // D s_31_12: cmp-lt s_31_11 s_31_10
        let s_31_12: bool = ((s_31_11) < (s_31_10));
        // N s_31_13: assert s_31_12
        let s_31_13: () = assert!(s_31_12);
        // C s_31_14: const #54s : i
        let s_31_14: i128 = 54;
        // D s_31_15: read-var descriptor:bv
        let s_31_15: Bits = fn_state.descriptor;
        // C s_31_16: const #1u : u64
        let s_31_16: u64 = 1;
        // D s_31_17: bit-extract s_31_15 s_31_14 s_31_16
        let s_31_17: Bits = (Bits::new(
            ((s_31_15) >> (s_31_14)).value(),
            u16::try_from(s_31_16).unwrap(),
        ));
        // D s_31_18: cast reint s_31_17 -> u8
        let s_31_18: bool = ((s_31_17.value()) != 0);
        // C s_31_19: const #0s : i
        let s_31_19: i128 = 0;
        // C s_31_20: const #0u : u64
        let s_31_20: u64 = 0;
        // D s_31_21: cast zx s_31_18 -> u64
        let s_31_21: u64 = (s_31_18 as u64);
        // C s_31_22: const #1u : u64
        let s_31_22: u64 = 1;
        // D s_31_23: and s_31_21 s_31_22
        let s_31_23: u64 = ((s_31_21) & (s_31_22));
        // D s_31_24: cmp-eq s_31_23 s_31_22
        let s_31_24: bool = ((s_31_23) == (s_31_22));
        // D s_31_25: lsl s_31_21 s_31_19
        let s_31_25: u64 = s_31_21 << s_31_19;
        // D s_31_26: or s_31_20 s_31_25
        let s_31_26: u64 = ((s_31_20) | (s_31_25));
        // D s_31_27: cmpl s_31_25
        let s_31_27: u64 = !s_31_25;
        // D s_31_28: and s_31_20 s_31_27
        let s_31_28: u64 = ((s_31_20) & (s_31_27));
        // D s_31_29: select s_31_24 s_31_26 s_31_28
        let s_31_29: u64 = if s_31_24 { s_31_26 } else { s_31_28 };
        // D s_31_30: cast trunc s_31_29 -> u8
        let s_31_30: bool = ((s_31_29) != 0);
        // D s_31_31: write-var permissions.5 <= s_31_30
        fn_state.permissions._5 = s_31_30;
        // N s_31_32: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_32_0: read-var walkparams.22:struct
        let s_32_0: bool = fn_state.walkparams._22;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#17885 <= s_32_4
        fn_state.gs_17885 = s_32_4;
        // N s_32_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call EL2Enabled(s_33_0)
        let s_33_1: bool = EL2Enabled(state, tracer, s_33_0);
        // D s_33_2: write-var gs#17884 <= s_33_1
        fn_state.gs_17884 = s_33_1;
        // N s_33_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_34_0: read-var walkparams.3:struct
        let s_34_0: bool = fn_state.walkparams._3;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // N s_34_5: branch s_34_4 b37 b35
        if s_34_4 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_35_0: read-var descriptor:bv
        let s_35_0: Bits = fn_state.descriptor;
        // D s_35_1: size-of s_35_0
        let s_35_1: u16 = s_35_0.length();
        // D s_35_2: cast zx s_35_1 -> i
        let s_35_2: i128 = (i128::try_from(s_35_1).unwrap());
        // C s_35_3: const #54s : i
        let s_35_3: i128 = 54;
        // D s_35_4: cmp-lt s_35_3 s_35_2
        let s_35_4: bool = ((s_35_3) < (s_35_2));
        // N s_35_5: assert s_35_4
        let s_35_5: () = assert!(s_35_4);
        // C s_35_6: const #53s : i
        let s_35_6: i128 = 53;
        // D s_35_7: read-var descriptor:bv
        let s_35_7: Bits = fn_state.descriptor;
        // C s_35_8: const #1s : i64
        let s_35_8: i64 = 1;
        // C s_35_9: cast zx s_35_8 -> i
        let s_35_9: i128 = (i128::try_from(s_35_8).unwrap());
        // C s_35_10: const #1s : i
        let s_35_10: i128 = 1;
        // C s_35_11: add s_35_10 s_35_9
        let s_35_11: i128 = (s_35_10 + s_35_9);
        // D s_35_12: bit-extract s_35_7 s_35_6 s_35_11
        let s_35_12: Bits = (Bits::new(
            ((s_35_7) >> (s_35_6)).value(),
            u16::try_from(s_35_11).unwrap(),
        ));
        // D s_35_13: cast reint s_35_12 -> u8
        let s_35_13: u8 = (s_35_12.value() as u8);
        // C s_35_14: const #51s : i
        let s_35_14: i128 = 51;
        // D s_35_15: read-var descriptor:bv
        let s_35_15: Bits = fn_state.descriptor;
        // C s_35_16: const #1u : u64
        let s_35_16: u64 = 1;
        // D s_35_17: bit-extract s_35_15 s_35_14 s_35_16
        let s_35_17: Bits = (Bits::new(
            ((s_35_15) >> (s_35_14)).value(),
            u16::try_from(s_35_16).unwrap(),
        ));
        // D s_35_18: cast reint s_35_17 -> u8
        let s_35_18: bool = ((s_35_17.value()) != 0);
        // C s_35_19: const #0s : i
        let s_35_19: i128 = 0;
        // C s_35_20: const #0u : u64
        let s_35_20: u64 = 0;
        // D s_35_21: cast zx s_35_18 -> u64
        let s_35_21: u64 = (s_35_18 as u64);
        // C s_35_22: const #1u : u64
        let s_35_22: u64 = 1;
        // D s_35_23: and s_35_21 s_35_22
        let s_35_23: u64 = ((s_35_21) & (s_35_22));
        // D s_35_24: cmp-eq s_35_23 s_35_22
        let s_35_24: bool = ((s_35_23) == (s_35_22));
        // D s_35_25: lsl s_35_21 s_35_19
        let s_35_25: u64 = s_35_21 << s_35_19;
        // D s_35_26: or s_35_20 s_35_25
        let s_35_26: u64 = ((s_35_20) | (s_35_25));
        // D s_35_27: cmpl s_35_25
        let s_35_27: u64 = !s_35_25;
        // D s_35_28: and s_35_20 s_35_27
        let s_35_28: u64 = ((s_35_20) & (s_35_27));
        // D s_35_29: select s_35_24 s_35_26 s_35_28
        let s_35_29: u64 = if s_35_24 { s_35_26 } else { s_35_28 };
        // D s_35_30: cast trunc s_35_29 -> u8
        let s_35_30: bool = ((s_35_29) != 0);
        // D s_35_31: cast zx s_35_13 -> bv
        let s_35_31: Bits = Bits::new(s_35_13 as u128, 2u16);
        // D s_35_32: cast zx s_35_30 -> bv
        let s_35_32: Bits = Bits::new(s_35_30 as u128, 1u16);
        // D s_35_33: cast reint s_35_31 -> u128
        let s_35_33: u128 = (s_35_31.value() as u128);
        // D s_35_34: size-of s_35_31
        let s_35_34: u16 = s_35_31.length();
        // D s_35_35: cast reint s_35_32 -> u128
        let s_35_35: u128 = (s_35_32.value() as u128);
        // D s_35_36: size-of s_35_32
        let s_35_36: u16 = s_35_32.length();
        // D s_35_37: lsl s_35_33 s_35_36
        let s_35_37: u128 = s_35_33 << s_35_36;
        // D s_35_38: or s_35_37 s_35_35
        let s_35_38: u128 = ((s_35_37) | (s_35_35));
        // D s_35_39: add s_35_34 s_35_36
        let s_35_39: u16 = (s_35_34 + s_35_36);
        // D s_35_40: create-bits s_35_38 s_35_39
        let s_35_40: Bits = Bits::new(s_35_38, s_35_39);
        // D s_35_41: cast reint s_35_40 -> u8
        let s_35_41: u8 = (s_35_40.value() as u8);
        // C s_35_42: const #6s : i
        let s_35_42: i128 = 6;
        // D s_35_43: read-var descriptor:bv
        let s_35_43: Bits = fn_state.descriptor;
        // C s_35_44: const #1u : u64
        let s_35_44: u64 = 1;
        // D s_35_45: bit-extract s_35_43 s_35_42 s_35_44
        let s_35_45: Bits = (Bits::new(
            ((s_35_43) >> (s_35_42)).value(),
            u16::try_from(s_35_44).unwrap(),
        ));
        // D s_35_46: cast reint s_35_45 -> u8
        let s_35_46: bool = ((s_35_45.value()) != 0);
        // C s_35_47: const #0s : i
        let s_35_47: i128 = 0;
        // C s_35_48: const #0u : u64
        let s_35_48: u64 = 0;
        // D s_35_49: cast zx s_35_46 -> u64
        let s_35_49: u64 = (s_35_46 as u64);
        // C s_35_50: const #1u : u64
        let s_35_50: u64 = 1;
        // D s_35_51: and s_35_49 s_35_50
        let s_35_51: u64 = ((s_35_49) & (s_35_50));
        // D s_35_52: cmp-eq s_35_51 s_35_50
        let s_35_52: bool = ((s_35_51) == (s_35_50));
        // D s_35_53: lsl s_35_49 s_35_47
        let s_35_53: u64 = s_35_49 << s_35_47;
        // D s_35_54: or s_35_48 s_35_53
        let s_35_54: u64 = ((s_35_48) | (s_35_53));
        // D s_35_55: cmpl s_35_53
        let s_35_55: u64 = !s_35_53;
        // D s_35_56: and s_35_48 s_35_55
        let s_35_56: u64 = ((s_35_48) & (s_35_55));
        // D s_35_57: select s_35_52 s_35_54 s_35_56
        let s_35_57: u64 = if s_35_52 { s_35_54 } else { s_35_56 };
        // D s_35_58: cast trunc s_35_57 -> u8
        let s_35_58: bool = ((s_35_57) != 0);
        // D s_35_59: cast zx s_35_41 -> bv
        let s_35_59: Bits = Bits::new(s_35_41 as u128, 3u16);
        // D s_35_60: cast zx s_35_58 -> bv
        let s_35_60: Bits = Bits::new(s_35_58 as u128, 1u16);
        // D s_35_61: cast reint s_35_59 -> u128
        let s_35_61: u128 = (s_35_59.value() as u128);
        // D s_35_62: size-of s_35_59
        let s_35_62: u16 = s_35_59.length();
        // D s_35_63: cast reint s_35_60 -> u128
        let s_35_63: u128 = (s_35_60.value() as u128);
        // D s_35_64: size-of s_35_60
        let s_35_64: u16 = s_35_60.length();
        // D s_35_65: lsl s_35_61 s_35_64
        let s_35_65: u128 = s_35_61 << s_35_64;
        // D s_35_66: or s_35_65 s_35_63
        let s_35_66: u128 = ((s_35_65) | (s_35_63));
        // D s_35_67: add s_35_62 s_35_64
        let s_35_67: u16 = (s_35_62 + s_35_64);
        // D s_35_68: create-bits s_35_66 s_35_67
        let s_35_68: Bits = Bits::new(s_35_66, s_35_67);
        // D s_35_69: cast reint s_35_68 -> u8
        let s_35_69: u8 = (s_35_68.value() as u8);
        // D s_35_70: write-var pi_index <= s_35_69
        fn_state.pi_index = s_35_69;
        // N s_35_71: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_36_0: read-var pi_index:u8
        let s_36_0: u8 = fn_state.pi_index;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 4u16);
        // D s_36_2: cast zx s_36_1 -> i
        let s_36_2: i128 = (s_36_1.value() as i128);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: const #4s : i
        let s_36_4: i128 = 4;
        // D s_36_5: cast zx s_36_3 -> i
        let s_36_5: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_6: mul s_36_4 s_36_5
        let s_36_6: i128 = ((s_36_4) * (s_36_5));
        // D s_36_7: cast reint s_36_6 -> i64
        let s_36_7: i64 = (s_36_6 as i64);
        // D s_36_8: read-var walkparams.25:struct
        let s_36_8: ProductType5c790c8ef59cc8b2 = fn_state.walkparams._25;
        // D s_36_9: write-var ga#13389 <= s_36_8
        fn_state.ga_13389 = s_36_8;
        // D s_36_10: read-var ga#13389.0:struct
        let s_36_10: u64 = fn_state.ga_13389._0;
        // C s_36_11: const #4s : i
        let s_36_11: i128 = 4;
        // D s_36_12: cast zx s_36_10 -> bv
        let s_36_12: Bits = Bits::new(s_36_10 as u128, 64u16);
        // D s_36_13: cast zx s_36_7 -> i
        let s_36_13: i128 = (i128::try_from(s_36_7).unwrap());
        // D s_36_14: bit-extract s_36_12 s_36_13 s_36_11
        let s_36_14: Bits = (Bits::new(
            ((s_36_12) >> (s_36_13)).value(),
            u16::try_from(s_36_11).unwrap(),
        ));
        // D s_36_15: cast reint s_36_14 -> u8
        let s_36_15: u8 = (s_36_14.value() as u8);
        // D s_36_16: write-var permissions.4 <= s_36_15
        fn_state.permissions._4 = s_36_15;
        // D s_36_17: read-var walkparams.26:struct
        let s_36_17: ProductType5c790c8ef59cc8b2 = fn_state.walkparams._26;
        // D s_36_18: write-var ga#13391 <= s_36_17
        fn_state.ga_13391 = s_36_17;
        // D s_36_19: read-var ga#13391.0:struct
        let s_36_19: u64 = fn_state.ga_13391._0;
        // C s_36_20: const #4s : i
        let s_36_20: i128 = 4;
        // D s_36_21: cast zx s_36_19 -> bv
        let s_36_21: Bits = Bits::new(s_36_19 as u128, 64u16);
        // D s_36_22: cast zx s_36_7 -> i
        let s_36_22: i128 = (i128::try_from(s_36_7).unwrap());
        // D s_36_23: bit-extract s_36_21 s_36_22 s_36_20
        let s_36_23: Bits = (Bits::new(
            ((s_36_21) >> (s_36_22)).value(),
            u16::try_from(s_36_20).unwrap(),
        ));
        // D s_36_24: cast reint s_36_23 -> u8
        let s_36_24: u8 = (s_36_23.value() as u8);
        // D s_36_25: write-var permissions.14 <= s_36_24
        fn_state.permissions._14 = s_36_24;
        // D s_36_26: read-var descriptor:bv
        let s_36_26: Bits = fn_state.descriptor;
        // D s_36_27: size-of s_36_26
        let s_36_27: u16 = s_36_26.length();
        // D s_36_28: cast zx s_36_27 -> i
        let s_36_28: i128 = (i128::try_from(s_36_27).unwrap());
        // C s_36_29: const #7s : i
        let s_36_29: i128 = 7;
        // D s_36_30: cmp-lt s_36_29 s_36_28
        let s_36_30: bool = ((s_36_29) < (s_36_28));
        // N s_36_31: assert s_36_30
        let s_36_31: () = assert!(s_36_30);
        // C s_36_32: const #7s : i
        let s_36_32: i128 = 7;
        // D s_36_33: read-var descriptor:bv
        let s_36_33: Bits = fn_state.descriptor;
        // C s_36_34: const #1u : u64
        let s_36_34: u64 = 1;
        // D s_36_35: bit-extract s_36_33 s_36_32 s_36_34
        let s_36_35: Bits = (Bits::new(
            ((s_36_33) >> (s_36_32)).value(),
            u16::try_from(s_36_34).unwrap(),
        ));
        // D s_36_36: cast reint s_36_35 -> u8
        let s_36_36: bool = ((s_36_35.value()) != 0);
        // C s_36_37: const #0s : i
        let s_36_37: i128 = 0;
        // C s_36_38: const #0u : u64
        let s_36_38: u64 = 0;
        // D s_36_39: cast zx s_36_36 -> u64
        let s_36_39: u64 = (s_36_36 as u64);
        // C s_36_40: const #1u : u64
        let s_36_40: u64 = 1;
        // D s_36_41: and s_36_39 s_36_40
        let s_36_41: u64 = ((s_36_39) & (s_36_40));
        // D s_36_42: cmp-eq s_36_41 s_36_40
        let s_36_42: bool = ((s_36_41) == (s_36_40));
        // D s_36_43: lsl s_36_39 s_36_37
        let s_36_43: u64 = s_36_39 << s_36_37;
        // D s_36_44: or s_36_38 s_36_43
        let s_36_44: u64 = ((s_36_38) | (s_36_43));
        // D s_36_45: cmpl s_36_43
        let s_36_45: u64 = !s_36_43;
        // D s_36_46: and s_36_38 s_36_45
        let s_36_46: u64 = ((s_36_38) & (s_36_45));
        // D s_36_47: select s_36_42 s_36_44 s_36_46
        let s_36_47: u64 = if s_36_42 { s_36_44 } else { s_36_46 };
        // D s_36_48: cast trunc s_36_47 -> u8
        let s_36_48: bool = ((s_36_47) != 0);
        // D s_36_49: write-var permissions.2 <= s_36_48
        fn_state.permissions._2 = s_36_48;
        // N s_36_50: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypebf05c51f33174538 {
        // D s_37_0: read-var descriptor:bv
        let s_37_0: Bits = fn_state.descriptor;
        // D s_37_1: size-of s_37_0
        let s_37_1: u16 = s_37_0.length();
        // D s_37_2: cast zx s_37_1 -> i
        let s_37_2: i128 = (i128::try_from(s_37_1).unwrap());
        // C s_37_3: const #118s : i
        let s_37_3: i128 = 118;
        // D s_37_4: cmp-lt s_37_3 s_37_2
        let s_37_4: bool = ((s_37_3) < (s_37_2));
        // N s_37_5: assert s_37_4
        let s_37_5: () = assert!(s_37_4);
        // C s_37_6: const #115s : i
        let s_37_6: i128 = 115;
        // D s_37_7: read-var descriptor:bv
        let s_37_7: Bits = fn_state.descriptor;
        // C s_37_8: const #1s : i64
        let s_37_8: i64 = 1;
        // C s_37_9: cast zx s_37_8 -> i
        let s_37_9: i128 = (i128::try_from(s_37_8).unwrap());
        // C s_37_10: const #3s : i
        let s_37_10: i128 = 3;
        // C s_37_11: add s_37_10 s_37_9
        let s_37_11: i128 = (s_37_10 + s_37_9);
        // D s_37_12: bit-extract s_37_7 s_37_6 s_37_11
        let s_37_12: Bits = (Bits::new(
            ((s_37_7) >> (s_37_6)).value(),
            u16::try_from(s_37_11).unwrap(),
        ));
        // D s_37_13: cast reint s_37_12 -> u8
        let s_37_13: u8 = (s_37_12.value() as u8);
        // D s_37_14: write-var pi_index <= s_37_13
        fn_state.pi_index = s_37_13;
        // N s_37_15: jump b36
        return block_36(state, tracer, fn_state);
    }
}

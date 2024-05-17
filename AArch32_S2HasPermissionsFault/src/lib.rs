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
use HaveExtendedExecuteNeverExt::*;
use ConstrainUnpredictable::*;
use common::*;
pub fn AArch32_S2HasPermissionsFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeb05ce25a107f0c5e,
    perms: ProductTypebf05c51f33174538,
    memtype: u32,
    accdesc: ProductType9878976b5bcce9c9,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_21574: ProductType8b847afc727d5818,
        ux: bool,
        r: bool,
        ga_21572: ProductType8b847afc727d5818,
        gs_27959: bool,
        ga_21571: ProductType8b847afc727d5818,
        gs_27958: bool,
        return_value: bool,
        w: bool,
        gs_27957: bool,
        gs_27956: bool,
        gs_27960: bool,
        px: bool,
        ga_21570: u8,
        x: bool,
        ga_21573: ProductType8b847afc727d5818,
        walkparams: ProductTypeb05ce25a107f0c5e,
        perms: ProductTypebf05c51f33174538,
        memtype: u32,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        walkparams,
        perms,
        memtype,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var perms.7:struct
        let s_0_0: u8 = fn_state.perms._7;
        // C s_0_1: const #0s : i
        let s_0_1: i128 = 0;
        // D s_0_2: cast zx s_0_0 -> bv
        let s_0_2: Bits = Bits::new(s_0_0 as u128, 2u16);
        // C s_0_3: const #1u : u64
        let s_0_3: u64 = 1;
        // D s_0_4: bit-extract s_0_2 s_0_1 s_0_3
        let s_0_4: Bits = (Bits::new(
            ((s_0_2) >> (s_0_1)).value(),
            u16::try_from(s_0_3).unwrap(),
        ));
        // D s_0_5: cast reint s_0_4 -> u8
        let s_0_5: bool = ((s_0_4.value()) != 0);
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // C s_0_7: const #0u : u64
        let s_0_7: u64 = 0;
        // D s_0_8: cast zx s_0_5 -> u64
        let s_0_8: u64 = (s_0_5 as u64);
        // C s_0_9: const #1u : u64
        let s_0_9: u64 = 1;
        // D s_0_10: and s_0_8 s_0_9
        let s_0_10: u64 = ((s_0_8) & (s_0_9));
        // D s_0_11: cmp-eq s_0_10 s_0_9
        let s_0_11: bool = ((s_0_10) == (s_0_9));
        // D s_0_12: lsl s_0_8 s_0_6
        let s_0_12: u64 = s_0_8 << s_0_6;
        // D s_0_13: or s_0_7 s_0_12
        let s_0_13: u64 = ((s_0_7) | (s_0_12));
        // D s_0_14: cmpl s_0_12
        let s_0_14: u64 = !s_0_12;
        // D s_0_15: and s_0_7 s_0_14
        let s_0_15: u64 = ((s_0_7) & (s_0_14));
        // D s_0_16: select s_0_11 s_0_13 s_0_15
        let s_0_16: u64 = if s_0_11 { s_0_13 } else { s_0_15 };
        // D s_0_17: cast trunc s_0_16 -> u8
        let s_0_17: bool = ((s_0_16) != 0);
        // D s_0_18: write-var r <= s_0_17
        fn_state.r = s_0_17;
        // D s_0_19: read-var perms.7:struct
        let s_0_19: u8 = fn_state.perms._7;
        // C s_0_20: const #1s : i
        let s_0_20: i128 = 1;
        // D s_0_21: cast zx s_0_19 -> bv
        let s_0_21: Bits = Bits::new(s_0_19 as u128, 2u16);
        // C s_0_22: const #1u : u64
        let s_0_22: u64 = 1;
        // D s_0_23: bit-extract s_0_21 s_0_20 s_0_22
        let s_0_23: Bits = (Bits::new(
            ((s_0_21) >> (s_0_20)).value(),
            u16::try_from(s_0_22).unwrap(),
        ));
        // D s_0_24: cast reint s_0_23 -> u8
        let s_0_24: bool = ((s_0_23.value()) != 0);
        // C s_0_25: const #0s : i
        let s_0_25: i128 = 0;
        // C s_0_26: const #0u : u64
        let s_0_26: u64 = 0;
        // D s_0_27: cast zx s_0_24 -> u64
        let s_0_27: u64 = (s_0_24 as u64);
        // C s_0_28: const #1u : u64
        let s_0_28: u64 = 1;
        // D s_0_29: and s_0_27 s_0_28
        let s_0_29: u64 = ((s_0_27) & (s_0_28));
        // D s_0_30: cmp-eq s_0_29 s_0_28
        let s_0_30: bool = ((s_0_29) == (s_0_28));
        // D s_0_31: lsl s_0_27 s_0_25
        let s_0_31: u64 = s_0_27 << s_0_25;
        // D s_0_32: or s_0_26 s_0_31
        let s_0_32: u64 = ((s_0_26) | (s_0_31));
        // D s_0_33: cmpl s_0_31
        let s_0_33: u64 = !s_0_31;
        // D s_0_34: and s_0_26 s_0_33
        let s_0_34: u64 = ((s_0_26) & (s_0_33));
        // D s_0_35: select s_0_30 s_0_32 s_0_34
        let s_0_35: u64 = if s_0_30 { s_0_32 } else { s_0_34 };
        // D s_0_36: cast trunc s_0_35 -> u8
        let s_0_36: bool = ((s_0_35) != 0);
        // D s_0_37: write-var w <= s_0_36
        fn_state.w = s_0_36;
        // C s_0_38: const #() : ()
        let s_0_38: () = ();
        // S s_0_39: call HaveExtendedExecuteNeverExt(s_0_38)
        let s_0_39: bool = HaveExtendedExecuteNeverExt(state, tracer, s_0_38);
        // N s_0_40: branch s_0_39 b27 b1
        if s_0_39 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var perms.12:struct
        let s_1_0: bool = fn_state.perms._12;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // D s_1_2: not s_1_1
        let s_1_2: Bits = !s_1_1;
        // D s_1_3: cast reint s_1_2 -> u8
        let s_1_3: bool = ((s_1_2.value()) != 0);
        // D s_1_4: read-var r:u8
        let s_1_4: bool = fn_state.r;
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // D s_1_6: cast zx s_1_3 -> bv
        let s_1_6: Bits = Bits::new(s_1_3 as u128, 1u16);
        // D s_1_7: and s_1_5 s_1_6
        let s_1_7: Bits = ((s_1_5) & (s_1_6));
        // D s_1_8: cast reint s_1_7 -> u8
        let s_1_8: bool = ((s_1_7.value()) != 0);
        // D s_1_9: write-var x <= s_1_8
        fn_state.x = s_1_8;
        // N s_1_10: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var accdesc.1:struct
        let s_2_0: u32 = fn_state.accdesc._1;
        // C s_2_1: const #13u : u32
        let s_2_1: u32 = 13;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b20 b3
        if s_2_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var accdesc.1:struct
        let s_3_0: u32 = fn_state.accdesc._1;
        // C s_3_1: const #0u : u32
        let s_3_1: u32 = 0;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b13 b4
        if s_3_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var accdesc.1:struct
        let s_4_0: u32 = fn_state.accdesc._1;
        // C s_4_1: const #5u : u32
        let s_4_1: u32 = 5;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // N s_4_3: branch s_4_2 b12 b5
        if s_4_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var accdesc.1:struct
        let s_5_0: u32 = fn_state.accdesc._1;
        // C s_5_1: const #6u : u32
        let s_5_1: u32 = 6;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: write-var gs#27956 <= s_5_2
        fn_state.gs_27956 = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#27956:u8
        let s_6_0: bool = fn_state.gs_27956;
        // N s_6_1: branch s_6_0 b11 b7
        if s_6_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var accdesc.32:struct
        let s_7_0: bool = fn_state.accdesc._32;
        // N s_7_1: branch s_7_0 b10 b8
        if s_7_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var r:u8
        let s_8_0: bool = fn_state.r;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #0u : u8
        let s_8_2: bool = false;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: write-var return_value <= s_8_4
        fn_state.return_value = s_8_4;
        // N s_8_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var return_value:u8
        let s_9_0: bool = fn_state.return_value;
        // N s_9_1: return s_9_0
        return s_9_0;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var w:u8
        let s_10_0: bool = fn_state.w;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #0u : u8
        let s_10_2: bool = false;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: write-var return_value <= s_10_4
        fn_state.return_value = s_10_4;
        // N s_10_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var return_value <= s_11_0
        fn_state.return_value = s_11_0;
        // N s_11_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#27956 <= s_12_0
        fn_state.gs_27956 = s_12_0;
        // N s_12_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #7u : u32
        let s_13_0: u32 = 7;
        // S s_13_1: call ConstrainUnpredictable(s_13_0)
        let s_13_1: u32 = ConstrainUnpredictable(state, tracer, s_13_0);
        // C s_13_2: const #12u : u32
        let s_13_2: u32 = 12;
        // S s_13_3: cmp-eq s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) == (s_13_2));
        // N s_13_4: branch s_13_3 b19 b14
        if s_13_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#27957 <= s_14_0
        fn_state.gs_27957 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var gs#27957:u8
        let s_15_0: bool = fn_state.gs_27957;
        // N s_15_1: branch s_15_0 b18 b16
        if s_15_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var x:u8
        let s_16_0: bool = fn_state.x;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #0u : u8
        let s_16_2: bool = false;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: write-var gs#27958 <= s_16_4
        fn_state.gs_27958 = s_16_4;
        // N s_16_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var gs#27958:u8
        let s_17_0: bool = fn_state.gs_27958;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#27958 <= s_18_0
        fn_state.gs_27958 = s_18_0;
        // N s_18_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var memtype:u32
        let s_19_0: u32 = fn_state.memtype;
        // C s_19_1: const #1u : u32
        let s_19_1: u32 = 1;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // D s_19_3: write-var gs#27957 <= s_19_2
        fn_state.gs_27957 = s_19_2;
        // N s_19_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var walkparams.15:struct
        let s_20_0: bool = fn_state.walkparams._15;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 1u16);
        // C s_20_2: const #1u : u8
        let s_20_2: bool = true;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // N s_20_5: branch s_20_4 b26 b21
        if s_20_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#27959 <= s_21_0
        fn_state.gs_27959 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var gs#27959:u8
        let s_22_0: bool = fn_state.gs_27959;
        // N s_22_1: branch s_22_0 b25 b23
        if s_22_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var r:u8
        let s_23_0: bool = fn_state.r;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #0u : u8
        let s_23_2: bool = false;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: write-var gs#27960 <= s_23_4
        fn_state.gs_27960 = s_23_4;
        // N s_23_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#27960:u8
        let s_24_0: bool = fn_state.gs_27960;
        // D s_24_1: write-var return_value <= s_24_0
        fn_state.return_value = s_24_0;
        // N s_24_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#27960 <= s_25_0
        fn_state.gs_27960 = s_25_0;
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var memtype:u32
        let s_26_0: u32 = fn_state.memtype;
        // C s_26_1: const #1u : u32
        let s_26_1: u32 = 1;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // D s_26_3: write-var gs#27959 <= s_26_2
        fn_state.gs_27959 = s_26_2;
        // N s_26_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var perms.12:struct
        let s_27_0: bool = fn_state.perms._12;
        // D s_27_1: read-var perms.13:struct
        let s_27_1: bool = fn_state.perms._13;
        // D s_27_2: cast zx s_27_0 -> bv
        let s_27_2: Bits = Bits::new(s_27_0 as u128, 1u16);
        // D s_27_3: cast zx s_27_1 -> bv
        let s_27_3: Bits = Bits::new(s_27_1 as u128, 1u16);
        // D s_27_4: cast reint s_27_2 -> u128
        let s_27_4: u128 = (s_27_2.value() as u128);
        // D s_27_5: size-of s_27_2
        let s_27_5: u16 = s_27_2.length();
        // D s_27_6: cast reint s_27_3 -> u128
        let s_27_6: u128 = (s_27_3.value() as u128);
        // D s_27_7: size-of s_27_3
        let s_27_7: u16 = s_27_3.length();
        // D s_27_8: lsl s_27_4 s_27_7
        let s_27_8: u128 = s_27_4 << s_27_7;
        // D s_27_9: or s_27_8 s_27_6
        let s_27_9: u128 = ((s_27_8) | (s_27_6));
        // D s_27_10: add s_27_5 s_27_7
        let s_27_10: u16 = (s_27_5 + s_27_7);
        // D s_27_11: create-bits s_27_9 s_27_10
        let s_27_11: Bits = Bits::new(s_27_9, s_27_10);
        // D s_27_12: cast reint s_27_11 -> u8
        let s_27_12: u8 = (s_27_11.value() as u8);
        // D s_27_13: write-var ga#21570 <= s_27_12
        fn_state.ga_21570 = s_27_12;
        // D s_27_14: read-var ga#21570:u8
        let s_27_14: u8 = fn_state.ga_21570;
        // D s_27_15: cast zx s_27_14 -> bv
        let s_27_15: Bits = Bits::new(s_27_14 as u128, 2u16);
        // C s_27_16: const #0u : u8
        let s_27_16: u8 = 0;
        // C s_27_17: cast zx s_27_16 -> bv
        let s_27_17: Bits = Bits::new(s_27_16 as u128, 2u16);
        // D s_27_18: cmp-eq s_27_15 s_27_17
        let s_27_18: bool = ((s_27_15) == (s_27_17));
        // D s_27_19: not s_27_18
        let s_27_19: bool = !s_27_18;
        // N s_27_20: branch s_27_19 b33 b28
        if s_27_19 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var r:u8
        let s_28_0: bool = fn_state.r;
        // D s_28_1: read-var r:u8
        let s_28_1: bool = fn_state.r;
        // D s_28_2: create-product struct = ["s_28_0", "s_28_1"]
        let s_28_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_28_0,
            _1: s_28_1,
        };
        // D s_28_3: write-var ga#21571 <= s_28_2
        fn_state.ga_21571 = s_28_2;
        // D s_28_4: read-var ga#21571.0:struct
        let s_28_4: bool = fn_state.ga_21571._0;
        // D s_28_5: read-var ga#21571.1:struct
        let s_28_5: bool = fn_state.ga_21571._1;
        // D s_28_6: write-var px <= s_28_4
        fn_state.px = s_28_4;
        // D s_28_7: write-var ux <= s_28_5
        fn_state.ux = s_28_5;
        // N s_28_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_29_0: read-var accdesc.8:struct
        let s_29_0: u8 = fn_state.accdesc._8;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 2u16);
        // C s_29_2: const #448u : u32
        let s_29_2: u32 = 448;
        // D s_29_3: read-reg s_29_2:u8
        let s_29_3: u8 = {
            let value = state.read_register::<u8>(s_29_2 as isize);
            tracer.read_register(s_29_2 as isize, value);
            value
        };
        // D s_29_4: cast zx s_29_3 -> bv
        let s_29_4: Bits = Bits::new(s_29_3 as u128, 2u16);
        // D s_29_5: cmp-eq s_29_1 s_29_4
        let s_29_5: bool = ((s_29_1) == (s_29_4));
        // N s_29_6: branch s_29_5 b32 b30
        if s_29_5 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var px:u8
        let s_30_0: bool = fn_state.px;
        // D s_30_1: write-var x <= s_30_0
        fn_state.x = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_31_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_32_0: read-var ux:u8
        let s_32_0: bool = fn_state.ux;
        // D s_32_1: write-var x <= s_32_0
        fn_state.x = s_32_0;
        // N s_32_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var ga#21570:u8
        let s_33_0: u8 = fn_state.ga_21570;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 2u16);
        // C s_33_2: const #1u : u8
        let s_33_2: u8 = 1;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 2u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: not s_33_4
        let s_33_5: bool = !s_33_4;
        // N s_33_6: branch s_33_5 b35 b34
        if s_33_5 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: read-var r:u8
        let s_34_1: bool = fn_state.r;
        // D s_34_2: create-product struct = ["s_34_0", "s_34_1"]
        let s_34_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_34_0,
            _1: s_34_1,
        };
        // D s_34_3: write-var ga#21572 <= s_34_2
        fn_state.ga_21572 = s_34_2;
        // D s_34_4: read-var ga#21572.0:struct
        let s_34_4: bool = fn_state.ga_21572._0;
        // D s_34_5: read-var ga#21572.1:struct
        let s_34_5: bool = fn_state.ga_21572._1;
        // D s_34_6: write-var px <= s_34_4
        fn_state.px = s_34_4;
        // D s_34_7: write-var ux <= s_34_5
        fn_state.ux = s_34_5;
        // N s_34_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_35_0: read-var ga#21570:u8
        let s_35_0: u8 = fn_state.ga_21570;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 2u16);
        // C s_35_2: const #2u : u8
        let s_35_2: u8 = 2;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 2u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // D s_35_5: not s_35_4
        let s_35_5: bool = !s_35_4;
        // N s_35_6: branch s_35_5 b37 b36
        if s_35_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // C s_36_1: const #0u : u8
        let s_36_1: bool = false;
        // D s_36_2: create-product struct = ["s_36_0", "s_36_1"]
        let s_36_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_36_0,
            _1: s_36_1,
        };
        // D s_36_3: write-var ga#21573 <= s_36_2
        fn_state.ga_21573 = s_36_2;
        // D s_36_4: read-var ga#21573.0:struct
        let s_36_4: bool = fn_state.ga_21573._0;
        // D s_36_5: read-var ga#21573.1:struct
        let s_36_5: bool = fn_state.ga_21573._1;
        // D s_36_6: write-var px <= s_36_4
        fn_state.px = s_36_4;
        // D s_36_7: write-var ux <= s_36_5
        fn_state.ux = s_36_5;
        // N s_36_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_37_0: read-var r:u8
        let s_37_0: bool = fn_state.r;
        // C s_37_1: const #0u : u8
        let s_37_1: bool = false;
        // D s_37_2: create-product struct = ["s_37_0", "s_37_1"]
        let s_37_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_37_0,
            _1: s_37_1,
        };
        // D s_37_3: write-var ga#21574 <= s_37_2
        fn_state.ga_21574 = s_37_2;
        // D s_37_4: read-var ga#21574.0:struct
        let s_37_4: bool = fn_state.ga_21574._0;
        // D s_37_5: read-var ga#21574.1:struct
        let s_37_5: bool = fn_state.ga_21574._1;
        // D s_37_6: write-var px <= s_37_4
        fn_state.px = s_37_4;
        // D s_37_7: write-var ux <= s_37_5
        fn_state.ux = s_37_5;
        // N s_37_8: jump b29
        return block_29(state, tracer, fn_state);
    }
}

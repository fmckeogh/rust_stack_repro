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
use UsingAArch32::*;
use ThisInstrLength::*;
use Unreachable::*;
use common::*;
pub fn AArch64_ExceptionClass<T: Tracer>(
    state: &mut State,
    tracer: &T,
    exceptype: u32,
    target_el: u8,
) -> ProductTypec1bd230b943b3b8c {
    #[derive(Default)]
    struct FunctionState {
        gs_5786: bool,
        from_32: bool,
        gs_5795: bool,
        gs_5793: bool,
        ec: i128,
        gs_5791: bool,
        gs_5802: bool,
        il_is_valid: bool,
        gs_5792: bool,
        gs_5789: bool,
        gs_5790: bool,
        gs_5788: bool,
        gs_5787: bool,
        gs_5794: bool,
        ecshadow_66: i128,
        il: bool,
        exceptype: u32,
        target_el: u8,
    }
    let fn_state = FunctionState {
        exceptype,
        target_el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // D s_0_1: write-var il_is_valid <= s_0_0
        fn_state.il_is_valid = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call UsingAArch32(s_0_2)
        let s_0_3: bool = UsingAArch32(state, tracer, s_0_2);
        // D s_0_4: write-var from_32 <= s_0_3
        fn_state.from_32 = s_0_3;
        // C s_0_5: const #0u : u32
        let s_0_5: u32 = 0;
        // D s_0_6: read-var exceptype:u32
        let s_0_6: u32 = fn_state.exceptype;
        // D s_0_7: cmp-eq s_0_5 s_0_6
        let s_0_7: bool = ((s_0_5) == (s_0_6));
        // D s_0_8: not s_0_7
        let s_0_8: bool = !s_0_7;
        // N s_0_9: branch s_0_8 b48 b1
        if s_0_8 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_1_0: const #0u : u8
        let s_1_0: u8 = 0;
        // C s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 8u16);
        // C s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: write-var ec <= s_1_2
        fn_state.ec = s_1_2;
        // C s_1_4: const #0u : u8
        let s_1_4: bool = false;
        // D s_1_5: write-var il_is_valid <= s_1_4
        fn_state.il_is_valid = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_2_0: const #32u : u8
        let s_2_0: u8 = 32;
        // C s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 8u16);
        // C s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // C s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: read-var ec:i
        let s_2_5: i128 = fn_state.ec;
        // D s_2_6: cmp-eq s_2_5 s_2_4
        let s_2_6: bool = ((s_2_5) == (s_2_4));
        // N s_2_7: branch s_2_6 b47 b3
        if s_2_6 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_3_0: const #36u : u8
        let s_3_0: u8 = 36;
        // C s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 8u16);
        // C s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // C s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var ec:i
        let s_3_5: i128 = fn_state.ec;
        // D s_3_6: cmp-eq s_3_5 s_3_4
        let s_3_6: bool = ((s_3_5) == (s_3_4));
        // N s_3_7: branch s_3_6 b46 b4
        if s_3_6 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_4_0: const #48u : u8
        let s_4_0: u8 = 48;
        // C s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 8u16);
        // C s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // C s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: read-var ec:i
        let s_4_5: i128 = fn_state.ec;
        // D s_4_6: cmp-eq s_4_5 s_4_4
        let s_4_6: bool = ((s_4_5) == (s_4_4));
        // N s_4_7: branch s_4_6 b45 b5
        if s_4_6 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_5_0: const #50u : u8
        let s_5_0: u8 = 50;
        // C s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 8u16);
        // C s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // C s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: read-var ec:i
        let s_5_5: i128 = fn_state.ec;
        // D s_5_6: cmp-eq s_5_5 s_5_4
        let s_5_6: bool = ((s_5_5) == (s_5_4));
        // N s_5_7: branch s_5_6 b44 b6
        if s_5_6 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_6_0: const #52u : u8
        let s_6_0: u8 = 52;
        // C s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 8u16);
        // C s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // C s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: read-var ec:i
        let s_6_5: i128 = fn_state.ec;
        // D s_6_6: cmp-eq s_6_5 s_6_4
        let s_6_6: bool = ((s_6_5) == (s_6_4));
        // D s_6_7: write-var gs#5786 <= s_6_6
        fn_state.gs_5786 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_7_0: read-var gs#5786:u8
        let s_7_0: bool = fn_state.gs_5786;
        // D s_7_1: write-var gs#5787 <= s_7_0
        fn_state.gs_5787 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_8_0: read-var gs#5787:u8
        let s_8_0: bool = fn_state.gs_5787;
        // D s_8_1: write-var gs#5788 <= s_8_0
        fn_state.gs_5788 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_9_0: read-var gs#5788:u8
        let s_9_0: bool = fn_state.gs_5788;
        // D s_9_1: write-var gs#5789 <= s_9_0
        fn_state.gs_5789 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_10_0: read-var gs#5789:u8
        let s_10_0: bool = fn_state.gs_5789;
        // N s_10_1: branch s_10_0 b43 b11
        if s_10_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#5790 <= s_11_0
        fn_state.gs_5790 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_12_0: read-var gs#5790:u8
        let s_12_0: bool = fn_state.gs_5790;
        // N s_12_1: branch s_12_0 b42 b13
        if s_12_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_14_0: const #17u : u8
        let s_14_0: u8 = 17;
        // C s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 8u16);
        // C s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (s_14_1.value() as i128);
        // C s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_5: read-var ec:i
        let s_14_5: i128 = fn_state.ec;
        // D s_14_6: cmp-eq s_14_5 s_14_4
        let s_14_6: bool = ((s_14_5) == (s_14_4));
        // N s_14_7: branch s_14_6 b41 b15
        if s_14_6 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_15_0: const #18u : u8
        let s_15_0: u8 = 18;
        // C s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 8u16);
        // C s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (s_15_1.value() as i128);
        // C s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_5: read-var ec:i
        let s_15_5: i128 = fn_state.ec;
        // D s_15_6: cmp-eq s_15_5 s_15_4
        let s_15_6: bool = ((s_15_5) == (s_15_4));
        // N s_15_7: branch s_15_6 b40 b16
        if s_15_6 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_16_0: const #19u : u8
        let s_16_0: u8 = 19;
        // C s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 8u16);
        // C s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (s_16_1.value() as i128);
        // C s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: read-var ec:i
        let s_16_5: i128 = fn_state.ec;
        // D s_16_6: cmp-eq s_16_5 s_16_4
        let s_16_6: bool = ((s_16_5) == (s_16_4));
        // N s_16_7: branch s_16_6 b39 b17
        if s_16_6 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_17_0: const #40u : u8
        let s_17_0: u8 = 40;
        // C s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 8u16);
        // C s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (s_17_1.value() as i128);
        // C s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_5: read-var ec:i
        let s_17_5: i128 = fn_state.ec;
        // D s_17_6: cmp-eq s_17_5 s_17_4
        let s_17_6: bool = ((s_17_5) == (s_17_4));
        // N s_17_7: branch s_17_6 b38 b18
        if s_17_6 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_18_0: const #56u : u8
        let s_18_0: u8 = 56;
        // C s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 8u16);
        // C s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (s_18_1.value() as i128);
        // C s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_5: read-var ec:i
        let s_18_5: i128 = fn_state.ec;
        // D s_18_6: cmp-eq s_18_5 s_18_4
        let s_18_6: bool = ((s_18_5) == (s_18_4));
        // D s_18_7: write-var gs#5791 <= s_18_6
        fn_state.gs_5791 = s_18_6;
        // N s_18_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_19_0: read-var gs#5791:u8
        let s_19_0: bool = fn_state.gs_5791;
        // D s_19_1: write-var gs#5792 <= s_19_0
        fn_state.gs_5792 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_20_0: read-var gs#5792:u8
        let s_20_0: bool = fn_state.gs_5792;
        // D s_20_1: write-var gs#5793 <= s_20_0
        fn_state.gs_5793 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_21_0: read-var gs#5793:u8
        let s_21_0: bool = fn_state.gs_5793;
        // D s_21_1: write-var gs#5794 <= s_21_0
        fn_state.gs_5794 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_22_0: read-var gs#5794:u8
        let s_22_0: bool = fn_state.gs_5794;
        // N s_22_1: branch s_22_0 b37 b23
        if s_22_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#5795 <= s_23_0
        fn_state.gs_5795 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_24_0: read-var gs#5795:u8
        let s_24_0: bool = fn_state.gs_5795;
        // N s_24_1: branch s_24_0 b36 b25
        if s_24_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_26_0: read-var ec:i
        let s_26_0: i128 = fn_state.ec;
        // D s_26_1: write-var ecshadow#66 <= s_26_0
        fn_state.ecshadow_66 = s_26_0;
        // D s_26_2: read-var il_is_valid:u8
        let s_26_2: bool = fn_state.il_is_valid;
        // N s_26_3: branch s_26_2 b32 b27
        if s_26_2 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var il <= s_27_0
        fn_state.il = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_28_0: read-var from_32:u8
        let s_28_0: bool = fn_state.from_32;
        // N s_28_1: branch s_28_0 b31 b29
        if s_28_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_29_0: read-var il:u8
        let s_29_0: bool = fn_state.il;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#5802 <= s_29_4
        fn_state.gs_5802 = s_29_4;
        // N s_29_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_30_0: read-var gs#5802:u8
        let s_30_0: bool = fn_state.gs_5802;
        // N s_30_1: assert s_30_0
        let s_30_1: () = assert!(s_30_0);
        // D s_30_2: read-var ecshadow#66:i
        let s_30_2: i128 = fn_state.ecshadow_66;
        // D s_30_3: read-var il:u8
        let s_30_3: bool = fn_state.il;
        // D s_30_4: create-product struct = ["s_30_2", "s_30_3"]
        let s_30_4: ProductTypec1bd230b943b3b8c = ProductTypec1bd230b943b3b8c {
            _0: s_30_2,
            _1: s_30_3,
        };
        // N s_30_5: return s_30_4
        return s_30_4;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#5802 <= s_31_0
        fn_state.gs_5802 = s_31_0;
        // N s_31_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call ThisInstrLength(s_32_0)
        let s_32_1: i128 = ThisInstrLength(state, tracer, s_32_0);
        // C s_32_2: const #32s : i
        let s_32_2: i128 = 32;
        // S s_32_3: cmp-eq s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) == (s_32_2));
        // N s_32_4: branch s_32_3 b35 b33
        if s_32_3 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var il <= s_33_0
        fn_state.il = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // N s_34_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var il <= s_35_0
        fn_state.il = s_35_0;
        // N s_35_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_36_0: const #4s : i
        let s_36_0: i128 = 4;
        // D s_36_1: read-var ec:i
        let s_36_1: i128 = fn_state.ec;
        // D s_36_2: add s_36_1 s_36_0
        let s_36_2: i128 = (s_36_1 + s_36_0);
        // D s_36_3: write-var ec <= s_36_2
        fn_state.ec = s_36_2;
        // N s_36_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_37_0: read-var from_32:u8
        let s_37_0: bool = fn_state.from_32;
        // D s_37_1: not s_37_0
        let s_37_1: bool = !s_37_0;
        // D s_37_2: write-var gs#5795 <= s_37_1
        fn_state.gs_5795 = s_37_1;
        // N s_37_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#5791 <= s_38_0
        fn_state.gs_5791 = s_38_0;
        // N s_38_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#5792 <= s_39_0
        fn_state.gs_5792 = s_39_0;
        // N s_39_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#5793 <= s_40_0
        fn_state.gs_5793 = s_40_0;
        // N s_40_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#5794 <= s_41_0
        fn_state.gs_5794 = s_41_0;
        // N s_41_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_42_0: const #1s : i
        let s_42_0: i128 = 1;
        // D s_42_1: read-var ec:i
        let s_42_1: i128 = fn_state.ec;
        // D s_42_2: add s_42_1 s_42_0
        let s_42_2: i128 = (s_42_1 + s_42_0);
        // D s_42_3: write-var ec <= s_42_2
        fn_state.ec = s_42_2;
        // N s_42_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_43_0: const #16975u : u32
        let s_43_0: u32 = 16975;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: read-var target_el:u8
        let s_43_2: u8 = fn_state.target_el;
        // D s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 2u16);
        // D s_43_4: cast zx s_43_1 -> bv
        let s_43_4: Bits = Bits::new(s_43_1 as u128, 2u16);
        // D s_43_5: cmp-eq s_43_3 s_43_4
        let s_43_5: bool = ((s_43_3) == (s_43_4));
        // D s_43_6: write-var gs#5790 <= s_43_5
        fn_state.gs_5790 = s_43_5;
        // N s_43_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#5786 <= s_44_0
        fn_state.gs_5786 = s_44_0;
        // N s_44_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#5787 <= s_45_0
        fn_state.gs_5787 = s_45_0;
        // N s_45_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#5788 <= s_46_0
        fn_state.gs_5788 = s_46_0;
        // N s_46_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#5789 <= s_47_0
        fn_state.gs_5789 = s_47_0;
        // N s_47_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_48_0: const #1u : u32
        let s_48_0: u32 = 1;
        // D s_48_1: read-var exceptype:u32
        let s_48_1: u32 = fn_state.exceptype;
        // D s_48_2: cmp-eq s_48_0 s_48_1
        let s_48_2: bool = ((s_48_0) == (s_48_1));
        // D s_48_3: not s_48_2
        let s_48_3: bool = !s_48_2;
        // N s_48_4: branch s_48_3 b50 b49
        if s_48_3 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_49_0: const #1u : u8
        let s_49_0: u8 = 1;
        // C s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 8u16);
        // C s_49_2: cast zx s_49_1 -> i
        let s_49_2: i128 = (s_49_1.value() as i128);
        // D s_49_3: write-var ec <= s_49_2
        fn_state.ec = s_49_2;
        // N s_49_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_50_0: const #2u : u32
        let s_50_0: u32 = 2;
        // D s_50_1: read-var exceptype:u32
        let s_50_1: u32 = fn_state.exceptype;
        // D s_50_2: cmp-eq s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) == (s_50_1));
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // N s_50_4: branch s_50_3 b52 b51
        if s_50_3 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_51_0: const #3u : u8
        let s_51_0: u8 = 3;
        // C s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 8u16);
        // C s_51_2: cast zx s_51_1 -> i
        let s_51_2: i128 = (s_51_1.value() as i128);
        // D s_51_3: write-var ec <= s_51_2
        fn_state.ec = s_51_2;
        // D s_51_4: read-var from_32:u8
        let s_51_4: bool = fn_state.from_32;
        // N s_51_5: assert s_51_4
        let s_51_5: () = assert!(s_51_4);
        // N s_51_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_52_0: const #3u : u32
        let s_52_0: u32 = 3;
        // D s_52_1: read-var exceptype:u32
        let s_52_1: u32 = fn_state.exceptype;
        // D s_52_2: cmp-eq s_52_0 s_52_1
        let s_52_2: bool = ((s_52_0) == (s_52_1));
        // D s_52_3: not s_52_2
        let s_52_3: bool = !s_52_2;
        // N s_52_4: branch s_52_3 b54 b53
        if s_52_3 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_53_0: const #4u : u8
        let s_53_0: u8 = 4;
        // C s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 8u16);
        // C s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (s_53_1.value() as i128);
        // D s_53_3: write-var ec <= s_53_2
        fn_state.ec = s_53_2;
        // D s_53_4: read-var from_32:u8
        let s_53_4: bool = fn_state.from_32;
        // N s_53_5: assert s_53_4
        let s_53_5: () = assert!(s_53_4);
        // N s_53_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_54_0: const #4u : u32
        let s_54_0: u32 = 4;
        // D s_54_1: read-var exceptype:u32
        let s_54_1: u32 = fn_state.exceptype;
        // D s_54_2: cmp-eq s_54_0 s_54_1
        let s_54_2: bool = ((s_54_0) == (s_54_1));
        // D s_54_3: not s_54_2
        let s_54_3: bool = !s_54_2;
        // N s_54_4: branch s_54_3 b56 b55
        if s_54_3 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_55_0: const #5u : u8
        let s_55_0: u8 = 5;
        // C s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 8u16);
        // C s_55_2: cast zx s_55_1 -> i
        let s_55_2: i128 = (s_55_1.value() as i128);
        // D s_55_3: write-var ec <= s_55_2
        fn_state.ec = s_55_2;
        // D s_55_4: read-var from_32:u8
        let s_55_4: bool = fn_state.from_32;
        // N s_55_5: assert s_55_4
        let s_55_5: () = assert!(s_55_4);
        // N s_55_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_56_0: const #5u : u32
        let s_56_0: u32 = 5;
        // D s_56_1: read-var exceptype:u32
        let s_56_1: u32 = fn_state.exceptype;
        // D s_56_2: cmp-eq s_56_0 s_56_1
        let s_56_2: bool = ((s_56_0) == (s_56_1));
        // D s_56_3: not s_56_2
        let s_56_3: bool = !s_56_2;
        // N s_56_4: branch s_56_3 b58 b57
        if s_56_3 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_57_0: const #6u : u8
        let s_57_0: u8 = 6;
        // C s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 8u16);
        // C s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (s_57_1.value() as i128);
        // D s_57_3: write-var ec <= s_57_2
        fn_state.ec = s_57_2;
        // D s_57_4: read-var from_32:u8
        let s_57_4: bool = fn_state.from_32;
        // N s_57_5: assert s_57_4
        let s_57_5: () = assert!(s_57_4);
        // N s_57_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_58_0: const #7u : u32
        let s_58_0: u32 = 7;
        // D s_58_1: read-var exceptype:u32
        let s_58_1: u32 = fn_state.exceptype;
        // D s_58_2: cmp-eq s_58_0 s_58_1
        let s_58_2: bool = ((s_58_0) == (s_58_1));
        // D s_58_3: not s_58_2
        let s_58_3: bool = !s_58_2;
        // N s_58_4: branch s_58_3 b60 b59
        if s_58_3 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_59_0: const #7u : u8
        let s_59_0: u8 = 7;
        // C s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 8u16);
        // C s_59_2: cast zx s_59_1 -> i
        let s_59_2: i128 = (s_59_1.value() as i128);
        // D s_59_3: write-var ec <= s_59_2
        fn_state.ec = s_59_2;
        // N s_59_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_60_0: const #8u : u32
        let s_60_0: u32 = 8;
        // D s_60_1: read-var exceptype:u32
        let s_60_1: u32 = fn_state.exceptype;
        // D s_60_2: cmp-eq s_60_0 s_60_1
        let s_60_2: bool = ((s_60_0) == (s_60_1));
        // D s_60_3: not s_60_2
        let s_60_3: bool = !s_60_2;
        // N s_60_4: branch s_60_3 b62 b61
        if s_60_3 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_61_0: const #8u : u8
        let s_61_0: u8 = 8;
        // C s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 8u16);
        // C s_61_2: cast zx s_61_1 -> i
        let s_61_2: i128 = (s_61_1.value() as i128);
        // D s_61_3: write-var ec <= s_61_2
        fn_state.ec = s_61_2;
        // N s_61_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_62_0: const #10u : u32
        let s_62_0: u32 = 10;
        // D s_62_1: read-var exceptype:u32
        let s_62_1: u32 = fn_state.exceptype;
        // D s_62_2: cmp-eq s_62_0 s_62_1
        let s_62_2: bool = ((s_62_0) == (s_62_1));
        // D s_62_3: not s_62_2
        let s_62_3: bool = !s_62_2;
        // N s_62_4: branch s_62_3 b64 b63
        if s_62_3 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_63_0: const #9u : u8
        let s_63_0: u8 = 9;
        // C s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 8u16);
        // C s_63_2: cast zx s_63_1 -> i
        let s_63_2: i128 = (s_63_1.value() as i128);
        // D s_63_3: write-var ec <= s_63_2
        fn_state.ec = s_63_2;
        // N s_63_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_64_0: const #9u : u32
        let s_64_0: u32 = 9;
        // D s_64_1: read-var exceptype:u32
        let s_64_1: u32 = fn_state.exceptype;
        // D s_64_2: cmp-eq s_64_0 s_64_1
        let s_64_2: bool = ((s_64_0) == (s_64_1));
        // D s_64_3: not s_64_2
        let s_64_3: bool = !s_64_2;
        // N s_64_4: branch s_64_3 b66 b65
        if s_64_3 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_65_0: const #10u : u8
        let s_65_0: u8 = 10;
        // C s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 8u16);
        // C s_65_2: cast zx s_65_1 -> i
        let s_65_2: i128 = (s_65_1.value() as i128);
        // D s_65_3: write-var ec <= s_65_2
        fn_state.ec = s_65_2;
        // N s_65_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_66_0: const #34u : u32
        let s_66_0: u32 = 34;
        // D s_66_1: read-var exceptype:u32
        let s_66_1: u32 = fn_state.exceptype;
        // D s_66_2: cmp-eq s_66_0 s_66_1
        let s_66_2: bool = ((s_66_0) == (s_66_1));
        // D s_66_3: not s_66_2
        let s_66_3: bool = !s_66_2;
        // N s_66_4: branch s_66_3 b68 b67
        if s_66_3 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_67_0: const #27u : u8
        let s_67_0: u8 = 27;
        // C s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 8u16);
        // C s_67_2: cast zx s_67_1 -> i
        let s_67_2: i128 = (s_67_1.value() as i128);
        // D s_67_3: write-var ec <= s_67_2
        fn_state.ec = s_67_2;
        // N s_67_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_68_0: const #35u : u32
        let s_68_0: u32 = 35;
        // D s_68_1: read-var exceptype:u32
        let s_68_1: u32 = fn_state.exceptype;
        // D s_68_2: cmp-eq s_68_0 s_68_1
        let s_68_2: bool = ((s_68_0) == (s_68_1));
        // D s_68_3: not s_68_2
        let s_68_3: bool = !s_68_2;
        // N s_68_4: branch s_68_3 b70 b69
        if s_68_3 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_69_0: const #30u : u8
        let s_69_0: u8 = 30;
        // C s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 8u16);
        // C s_69_2: cast zx s_69_1 -> i
        let s_69_2: i128 = (s_69_1.value() as i128);
        // D s_69_3: write-var ec <= s_69_2
        fn_state.ec = s_69_2;
        // N s_69_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_70_0: const #6u : u32
        let s_70_0: u32 = 6;
        // D s_70_1: read-var exceptype:u32
        let s_70_1: u32 = fn_state.exceptype;
        // D s_70_2: cmp-eq s_70_0 s_70_1
        let s_70_2: bool = ((s_70_0) == (s_70_1));
        // D s_70_3: not s_70_2
        let s_70_3: bool = !s_70_2;
        // N s_70_4: branch s_70_3 b72 b71
        if s_70_3 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_71_0: const #12u : u8
        let s_71_0: u8 = 12;
        // C s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 8u16);
        // C s_71_2: cast zx s_71_1 -> i
        let s_71_2: i128 = (s_71_1.value() as i128);
        // D s_71_3: write-var ec <= s_71_2
        fn_state.ec = s_71_2;
        // D s_71_4: read-var from_32:u8
        let s_71_4: bool = fn_state.from_32;
        // N s_71_5: assert s_71_4
        let s_71_5: () = assert!(s_71_4);
        // N s_71_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_72_0: const #36u : u32
        let s_72_0: u32 = 36;
        // D s_72_1: read-var exceptype:u32
        let s_72_1: u32 = fn_state.exceptype;
        // D s_72_2: cmp-eq s_72_0 s_72_1
        let s_72_2: bool = ((s_72_0) == (s_72_1));
        // D s_72_3: not s_72_2
        let s_72_3: bool = !s_72_2;
        // N s_72_4: branch s_72_3 b74 b73
        if s_72_3 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_73_0: const #13u : u8
        let s_73_0: u8 = 13;
        // C s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 8u16);
        // C s_73_2: cast zx s_73_1 -> i
        let s_73_2: i128 = (s_73_1.value() as i128);
        // D s_73_3: write-var ec <= s_73_2
        fn_state.ec = s_73_2;
        // N s_73_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_74_0: const #11u : u32
        let s_74_0: u32 = 11;
        // D s_74_1: read-var exceptype:u32
        let s_74_1: u32 = fn_state.exceptype;
        // D s_74_2: cmp-eq s_74_0 s_74_1
        let s_74_2: bool = ((s_74_0) == (s_74_1));
        // D s_74_3: not s_74_2
        let s_74_3: bool = !s_74_2;
        // N s_74_4: branch s_74_3 b76 b75
        if s_74_3 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_75_0: const #14u : u8
        let s_75_0: u8 = 14;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 8u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // D s_75_3: write-var ec <= s_75_2
        fn_state.ec = s_75_2;
        // C s_75_4: const #0u : u8
        let s_75_4: bool = false;
        // D s_75_5: write-var il_is_valid <= s_75_4
        fn_state.il_is_valid = s_75_4;
        // N s_75_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_76_0: const #12u : u32
        let s_76_0: u32 = 12;
        // D s_76_1: read-var exceptype:u32
        let s_76_1: u32 = fn_state.exceptype;
        // D s_76_2: cmp-eq s_76_0 s_76_1
        let s_76_2: bool = ((s_76_0) == (s_76_1));
        // D s_76_3: not s_76_2
        let s_76_3: bool = !s_76_2;
        // N s_76_4: branch s_76_3 b78 b77
        if s_76_3 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_77_0: const #17u : u8
        let s_77_0: u8 = 17;
        // C s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 8u16);
        // C s_77_2: cast zx s_77_1 -> i
        let s_77_2: i128 = (s_77_1.value() as i128);
        // D s_77_3: write-var ec <= s_77_2
        fn_state.ec = s_77_2;
        // N s_77_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_78_0: const #13u : u32
        let s_78_0: u32 = 13;
        // D s_78_1: read-var exceptype:u32
        let s_78_1: u32 = fn_state.exceptype;
        // D s_78_2: cmp-eq s_78_0 s_78_1
        let s_78_2: bool = ((s_78_0) == (s_78_1));
        // D s_78_3: not s_78_2
        let s_78_3: bool = !s_78_2;
        // N s_78_4: branch s_78_3 b80 b79
        if s_78_3 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_79_0: const #18u : u8
        let s_79_0: u8 = 18;
        // C s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 8u16);
        // C s_79_2: cast zx s_79_1 -> i
        let s_79_2: i128 = (s_79_1.value() as i128);
        // D s_79_3: write-var ec <= s_79_2
        fn_state.ec = s_79_2;
        // N s_79_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_80_0: const #14u : u32
        let s_80_0: u32 = 14;
        // D s_80_1: read-var exceptype:u32
        let s_80_1: u32 = fn_state.exceptype;
        // D s_80_2: cmp-eq s_80_0 s_80_1
        let s_80_2: bool = ((s_80_0) == (s_80_1));
        // D s_80_3: not s_80_2
        let s_80_3: bool = !s_80_2;
        // N s_80_4: branch s_80_3 b82 b81
        if s_80_3 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_81_0: const #19u : u8
        let s_81_0: u8 = 19;
        // C s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 8u16);
        // C s_81_2: cast zx s_81_1 -> i
        let s_81_2: i128 = (s_81_1.value() as i128);
        // D s_81_3: write-var ec <= s_81_2
        fn_state.ec = s_81_2;
        // N s_81_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_82_0: const #15u : u32
        let s_82_0: u32 = 15;
        // D s_82_1: read-var exceptype:u32
        let s_82_1: u32 = fn_state.exceptype;
        // D s_82_2: cmp-eq s_82_0 s_82_1
        let s_82_2: bool = ((s_82_0) == (s_82_1));
        // D s_82_3: not s_82_2
        let s_82_3: bool = !s_82_2;
        // N s_82_4: branch s_82_3 b84 b83
        if s_82_3 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_83_0: const #24u : u8
        let s_83_0: u8 = 24;
        // C s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 8u16);
        // C s_83_2: cast zx s_83_1 -> i
        let s_83_2: i128 = (s_83_1.value() as i128);
        // D s_83_3: write-var ec <= s_83_2
        fn_state.ec = s_83_2;
        // D s_83_4: read-var from_32:u8
        let s_83_4: bool = fn_state.from_32;
        // D s_83_5: not s_83_4
        let s_83_5: bool = !s_83_4;
        // N s_83_6: assert s_83_5
        let s_83_6: () = assert!(s_83_5);
        // N s_83_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_84_0: const #39u : u32
        let s_84_0: u32 = 39;
        // D s_84_1: read-var exceptype:u32
        let s_84_1: u32 = fn_state.exceptype;
        // D s_84_2: cmp-eq s_84_0 s_84_1
        let s_84_2: bool = ((s_84_0) == (s_84_1));
        // D s_84_3: not s_84_2
        let s_84_3: bool = !s_84_2;
        // N s_84_4: branch s_84_3 b86 b85
        if s_84_3 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_85_0: const #20u : u8
        let s_85_0: u8 = 20;
        // C s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 8u16);
        // C s_85_2: cast zx s_85_1 -> i
        let s_85_2: i128 = (s_85_1.value() as i128);
        // D s_85_3: write-var ec <= s_85_2
        fn_state.ec = s_85_2;
        // D s_85_4: read-var from_32:u8
        let s_85_4: bool = fn_state.from_32;
        // D s_85_5: not s_85_4
        let s_85_5: bool = !s_85_4;
        // N s_85_6: assert s_85_5
        let s_85_6: () = assert!(s_85_5);
        // N s_85_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_86_0: const #32u : u32
        let s_86_0: u32 = 32;
        // D s_86_1: read-var exceptype:u32
        let s_86_1: u32 = fn_state.exceptype;
        // D s_86_2: cmp-eq s_86_0 s_86_1
        let s_86_2: bool = ((s_86_0) == (s_86_1));
        // D s_86_3: not s_86_2
        let s_86_3: bool = !s_86_2;
        // N s_86_4: branch s_86_3 b88 b87
        if s_86_3 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_87_0: const #25u : u8
        let s_87_0: u8 = 25;
        // C s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 8u16);
        // C s_87_2: cast zx s_87_1 -> i
        let s_87_2: i128 = (s_87_1.value() as i128);
        // D s_87_3: write-var ec <= s_87_2
        fn_state.ec = s_87_2;
        // D s_87_4: read-var from_32:u8
        let s_87_4: bool = fn_state.from_32;
        // D s_87_5: not s_87_4
        let s_87_5: bool = !s_87_4;
        // N s_87_6: assert s_87_5
        let s_87_6: () = assert!(s_87_5);
        // N s_87_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_88_0: const #16u : u32
        let s_88_0: u32 = 16;
        // D s_88_1: read-var exceptype:u32
        let s_88_1: u32 = fn_state.exceptype;
        // D s_88_2: cmp-eq s_88_0 s_88_1
        let s_88_2: bool = ((s_88_0) == (s_88_1));
        // D s_88_3: not s_88_2
        let s_88_3: bool = !s_88_2;
        // N s_88_4: branch s_88_3 b90 b89
        if s_88_3 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_89_0: const #26u : u8
        let s_89_0: u8 = 26;
        // C s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 8u16);
        // C s_89_2: cast zx s_89_1 -> i
        let s_89_2: i128 = (s_89_1.value() as i128);
        // D s_89_3: write-var ec <= s_89_2
        fn_state.ec = s_89_2;
        // D s_89_4: read-var from_32:u8
        let s_89_4: bool = fn_state.from_32;
        // D s_89_5: not s_89_4
        let s_89_5: bool = !s_89_4;
        // N s_89_6: assert s_89_5
        let s_89_6: () = assert!(s_89_5);
        // N s_89_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_90_0: const #21u : u32
        let s_90_0: u32 = 21;
        // D s_90_1: read-var exceptype:u32
        let s_90_1: u32 = fn_state.exceptype;
        // D s_90_2: cmp-eq s_90_0 s_90_1
        let s_90_2: bool = ((s_90_0) == (s_90_1));
        // D s_90_3: not s_90_2
        let s_90_3: bool = !s_90_2;
        // N s_90_4: branch s_90_3 b92 b91
        if s_90_3 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_91_0: const #28u : u8
        let s_91_0: u8 = 28;
        // C s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 8u16);
        // C s_91_2: cast zx s_91_1 -> i
        let s_91_2: i128 = (s_91_1.value() as i128);
        // D s_91_3: write-var ec <= s_91_2
        fn_state.ec = s_91_2;
        // D s_91_4: read-var from_32:u8
        let s_91_4: bool = fn_state.from_32;
        // D s_91_5: not s_91_4
        let s_91_5: bool = !s_91_4;
        // N s_91_6: assert s_91_5
        let s_91_6: () = assert!(s_91_5);
        // N s_91_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_92_0: const #33u : u32
        let s_92_0: u32 = 33;
        // D s_92_1: read-var exceptype:u32
        let s_92_1: u32 = fn_state.exceptype;
        // D s_92_2: cmp-eq s_92_0 s_92_1
        let s_92_2: bool = ((s_92_0) == (s_92_1));
        // D s_92_3: not s_92_2
        let s_92_3: bool = !s_92_2;
        // N s_92_4: branch s_92_3 b94 b93
        if s_92_3 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_93_0: const #29u : u8
        let s_93_0: u8 = 29;
        // C s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 8u16);
        // C s_93_2: cast zx s_93_1 -> i
        let s_93_2: i128 = (s_93_1.value() as i128);
        // D s_93_3: write-var ec <= s_93_2
        fn_state.ec = s_93_2;
        // D s_93_4: read-var from_32:u8
        let s_93_4: bool = fn_state.from_32;
        // D s_93_5: not s_93_4
        let s_93_5: bool = !s_93_4;
        // N s_93_6: assert s_93_5
        let s_93_6: () = assert!(s_93_5);
        // N s_93_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_94_0: const #17u : u32
        let s_94_0: u32 = 17;
        // D s_94_1: read-var exceptype:u32
        let s_94_1: u32 = fn_state.exceptype;
        // D s_94_2: cmp-eq s_94_0 s_94_1
        let s_94_2: bool = ((s_94_0) == (s_94_1));
        // D s_94_3: not s_94_2
        let s_94_3: bool = !s_94_2;
        // N s_94_4: branch s_94_3 b96 b95
        if s_94_3 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_95_0: const #32u : u8
        let s_95_0: u8 = 32;
        // C s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 8u16);
        // C s_95_2: cast zx s_95_1 -> i
        let s_95_2: i128 = (s_95_1.value() as i128);
        // D s_95_3: write-var ec <= s_95_2
        fn_state.ec = s_95_2;
        // C s_95_4: const #0u : u8
        let s_95_4: bool = false;
        // D s_95_5: write-var il_is_valid <= s_95_4
        fn_state.il_is_valid = s_95_4;
        // N s_95_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_96_0: const #18u : u32
        let s_96_0: u32 = 18;
        // D s_96_1: read-var exceptype:u32
        let s_96_1: u32 = fn_state.exceptype;
        // D s_96_2: cmp-eq s_96_0 s_96_1
        let s_96_2: bool = ((s_96_0) == (s_96_1));
        // D s_96_3: not s_96_2
        let s_96_3: bool = !s_96_2;
        // N s_96_4: branch s_96_3 b98 b97
        if s_96_3 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_97_0: const #34u : u8
        let s_97_0: u8 = 34;
        // C s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 8u16);
        // C s_97_2: cast zx s_97_1 -> i
        let s_97_2: i128 = (s_97_1.value() as i128);
        // D s_97_3: write-var ec <= s_97_2
        fn_state.ec = s_97_2;
        // C s_97_4: const #0u : u8
        let s_97_4: bool = false;
        // D s_97_5: write-var il_is_valid <= s_97_4
        fn_state.il_is_valid = s_97_4;
        // N s_97_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_98_0: const #19u : u32
        let s_98_0: u32 = 19;
        // D s_98_1: read-var exceptype:u32
        let s_98_1: u32 = fn_state.exceptype;
        // D s_98_2: cmp-eq s_98_0 s_98_1
        let s_98_2: bool = ((s_98_0) == (s_98_1));
        // D s_98_3: not s_98_2
        let s_98_3: bool = !s_98_2;
        // N s_98_4: branch s_98_3 b100 b99
        if s_98_3 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_99_0: const #36u : u8
        let s_99_0: u8 = 36;
        // C s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 8u16);
        // C s_99_2: cast zx s_99_1 -> i
        let s_99_2: i128 = (s_99_1.value() as i128);
        // D s_99_3: write-var ec <= s_99_2
        fn_state.ec = s_99_2;
        // N s_99_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_100_0: const #20u : u32
        let s_100_0: u32 = 20;
        // D s_100_1: read-var exceptype:u32
        let s_100_1: u32 = fn_state.exceptype;
        // D s_100_2: cmp-eq s_100_0 s_100_1
        let s_100_2: bool = ((s_100_0) == (s_100_1));
        // D s_100_3: not s_100_2
        let s_100_3: bool = !s_100_2;
        // N s_100_4: branch s_100_3 b102 b101
        if s_100_3 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_101_0: const #37u : u8
        let s_101_0: u8 = 37;
        // C s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 8u16);
        // C s_101_2: cast zx s_101_1 -> i
        let s_101_2: i128 = (s_101_1.value() as i128);
        // D s_101_3: write-var ec <= s_101_2
        fn_state.ec = s_101_2;
        // N s_101_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_102_0: const #22u : u32
        let s_102_0: u32 = 22;
        // D s_102_1: read-var exceptype:u32
        let s_102_1: u32 = fn_state.exceptype;
        // D s_102_2: cmp-eq s_102_0 s_102_1
        let s_102_2: bool = ((s_102_0) == (s_102_1));
        // D s_102_3: not s_102_2
        let s_102_3: bool = !s_102_2;
        // N s_102_4: branch s_102_3 b104 b103
        if s_102_3 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_103_0: const #38u : u8
        let s_103_0: u8 = 38;
        // C s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 8u16);
        // C s_103_2: cast zx s_103_1 -> i
        let s_103_2: i128 = (s_103_1.value() as i128);
        // D s_103_3: write-var ec <= s_103_2
        fn_state.ec = s_103_2;
        // C s_103_4: const #0u : u8
        let s_103_4: bool = false;
        // D s_103_5: write-var il_is_valid <= s_103_4
        fn_state.il_is_valid = s_103_4;
        // D s_103_6: read-var from_32:u8
        let s_103_6: bool = fn_state.from_32;
        // D s_103_7: not s_103_6
        let s_103_7: bool = !s_103_6;
        // N s_103_8: assert s_103_7
        let s_103_8: () = assert!(s_103_7);
        // N s_103_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_104_0: const #37u : u32
        let s_104_0: u32 = 37;
        // D s_104_1: read-var exceptype:u32
        let s_104_1: u32 = fn_state.exceptype;
        // D s_104_2: cmp-eq s_104_0 s_104_1
        let s_104_2: bool = ((s_104_0) == (s_104_1));
        // D s_104_3: not s_104_2
        let s_104_3: bool = !s_104_2;
        // N s_104_4: branch s_104_3 b106 b105
        if s_104_3 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_105_0: const #39u : u8
        let s_105_0: u8 = 39;
        // C s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 8u16);
        // C s_105_2: cast zx s_105_1 -> i
        let s_105_2: i128 = (s_105_1.value() as i128);
        // D s_105_3: write-var ec <= s_105_2
        fn_state.ec = s_105_2;
        // N s_105_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_106_0: const #38u : u32
        let s_106_0: u32 = 38;
        // D s_106_1: read-var exceptype:u32
        let s_106_1: u32 = fn_state.exceptype;
        // D s_106_2: cmp-eq s_106_0 s_106_1
        let s_106_2: bool = ((s_106_0) == (s_106_1));
        // D s_106_3: not s_106_2
        let s_106_3: bool = !s_106_2;
        // N s_106_4: branch s_106_3 b108 b107
        if s_106_3 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_107_0: const #45u : u8
        let s_107_0: u8 = 45;
        // C s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 8u16);
        // C s_107_2: cast zx s_107_1 -> i
        let s_107_2: i128 = (s_107_1.value() as i128);
        // D s_107_3: write-var ec <= s_107_2
        fn_state.ec = s_107_2;
        // D s_107_4: read-var from_32:u8
        let s_107_4: bool = fn_state.from_32;
        // D s_107_5: not s_107_4
        let s_107_5: bool = !s_107_4;
        // N s_107_6: assert s_107_5
        let s_107_6: () = assert!(s_107_5);
        // N s_107_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_108_0: const #23u : u32
        let s_108_0: u32 = 23;
        // D s_108_1: read-var exceptype:u32
        let s_108_1: u32 = fn_state.exceptype;
        // D s_108_2: cmp-eq s_108_0 s_108_1
        let s_108_2: bool = ((s_108_0) == (s_108_1));
        // D s_108_3: not s_108_2
        let s_108_3: bool = !s_108_2;
        // N s_108_4: branch s_108_3 b110 b109
        if s_108_3 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_109_0: const #40u : u8
        let s_109_0: u8 = 40;
        // C s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 8u16);
        // C s_109_2: cast zx s_109_1 -> i
        let s_109_2: i128 = (s_109_1.value() as i128);
        // D s_109_3: write-var ec <= s_109_2
        fn_state.ec = s_109_2;
        // N s_109_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_110_0: const #24u : u32
        let s_110_0: u32 = 24;
        // D s_110_1: read-var exceptype:u32
        let s_110_1: u32 = fn_state.exceptype;
        // D s_110_2: cmp-eq s_110_0 s_110_1
        let s_110_2: bool = ((s_110_0) == (s_110_1));
        // D s_110_3: not s_110_2
        let s_110_3: bool = !s_110_2;
        // N s_110_4: branch s_110_3 b112 b111
        if s_110_3 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_111_0: const #47u : u8
        let s_111_0: u8 = 47;
        // C s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 8u16);
        // C s_111_2: cast zx s_111_1 -> i
        let s_111_2: i128 = (s_111_1.value() as i128);
        // D s_111_3: write-var ec <= s_111_2
        fn_state.ec = s_111_2;
        // C s_111_4: const #0u : u8
        let s_111_4: bool = false;
        // D s_111_5: write-var il_is_valid <= s_111_4
        fn_state.il_is_valid = s_111_4;
        // N s_111_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_112_0: const #25u : u32
        let s_112_0: u32 = 25;
        // D s_112_1: read-var exceptype:u32
        let s_112_1: u32 = fn_state.exceptype;
        // D s_112_2: cmp-eq s_112_0 s_112_1
        let s_112_2: bool = ((s_112_0) == (s_112_1));
        // D s_112_3: not s_112_2
        let s_112_3: bool = !s_112_2;
        // N s_112_4: branch s_112_3 b114 b113
        if s_112_3 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_113_0: const #48u : u8
        let s_113_0: u8 = 48;
        // C s_113_1: cast zx s_113_0 -> bv
        let s_113_1: Bits = Bits::new(s_113_0 as u128, 8u16);
        // C s_113_2: cast zx s_113_1 -> i
        let s_113_2: i128 = (s_113_1.value() as i128);
        // D s_113_3: write-var ec <= s_113_2
        fn_state.ec = s_113_2;
        // C s_113_4: const #0u : u8
        let s_113_4: bool = false;
        // D s_113_5: write-var il_is_valid <= s_113_4
        fn_state.il_is_valid = s_113_4;
        // N s_113_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_114_0: const #26u : u32
        let s_114_0: u32 = 26;
        // D s_114_1: read-var exceptype:u32
        let s_114_1: u32 = fn_state.exceptype;
        // D s_114_2: cmp-eq s_114_0 s_114_1
        let s_114_2: bool = ((s_114_0) == (s_114_1));
        // D s_114_3: not s_114_2
        let s_114_3: bool = !s_114_2;
        // N s_114_4: branch s_114_3 b116 b115
        if s_114_3 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_115_0: const #50u : u8
        let s_115_0: u8 = 50;
        // C s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 8u16);
        // C s_115_2: cast zx s_115_1 -> i
        let s_115_2: i128 = (s_115_1.value() as i128);
        // D s_115_3: write-var ec <= s_115_2
        fn_state.ec = s_115_2;
        // C s_115_4: const #0u : u8
        let s_115_4: bool = false;
        // D s_115_5: write-var il_is_valid <= s_115_4
        fn_state.il_is_valid = s_115_4;
        // N s_115_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_116_0: const #27u : u32
        let s_116_0: u32 = 27;
        // D s_116_1: read-var exceptype:u32
        let s_116_1: u32 = fn_state.exceptype;
        // D s_116_2: cmp-eq s_116_0 s_116_1
        let s_116_2: bool = ((s_116_0) == (s_116_1));
        // D s_116_3: not s_116_2
        let s_116_3: bool = !s_116_2;
        // N s_116_4: branch s_116_3 b118 b117
        if s_116_3 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_117_0: const #52u : u8
        let s_117_0: u8 = 52;
        // C s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 8u16);
        // C s_117_2: cast zx s_117_1 -> i
        let s_117_2: i128 = (s_117_1.value() as i128);
        // D s_117_3: write-var ec <= s_117_2
        fn_state.ec = s_117_2;
        // C s_117_4: const #0u : u8
        let s_117_4: bool = false;
        // D s_117_5: write-var il_is_valid <= s_117_4
        fn_state.il_is_valid = s_117_4;
        // N s_117_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_118_0: const #28u : u32
        let s_118_0: u32 = 28;
        // D s_118_1: read-var exceptype:u32
        let s_118_1: u32 = fn_state.exceptype;
        // D s_118_2: cmp-eq s_118_0 s_118_1
        let s_118_2: bool = ((s_118_0) == (s_118_1));
        // D s_118_3: not s_118_2
        let s_118_3: bool = !s_118_2;
        // N s_118_4: branch s_118_3 b120 b119
        if s_118_3 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_119_0: const #53u : u8
        let s_119_0: u8 = 53;
        // C s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 8u16);
        // C s_119_2: cast zx s_119_1 -> i
        let s_119_2: i128 = (s_119_1.value() as i128);
        // D s_119_3: write-var ec <= s_119_2
        fn_state.ec = s_119_2;
        // C s_119_4: const #0u : u8
        let s_119_4: bool = false;
        // D s_119_5: write-var il_is_valid <= s_119_4
        fn_state.il_is_valid = s_119_4;
        // N s_119_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_120_0: const #29u : u32
        let s_120_0: u32 = 29;
        // D s_120_1: read-var exceptype:u32
        let s_120_1: u32 = fn_state.exceptype;
        // D s_120_2: cmp-eq s_120_0 s_120_1
        let s_120_2: bool = ((s_120_0) == (s_120_1));
        // D s_120_3: not s_120_2
        let s_120_3: bool = !s_120_2;
        // N s_120_4: branch s_120_3 b122 b121
        if s_120_3 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_121_0: const #56u : u8
        let s_121_0: u8 = 56;
        // C s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 8u16);
        // C s_121_2: cast zx s_121_1 -> i
        let s_121_2: i128 = (s_121_1.value() as i128);
        // D s_121_3: write-var ec <= s_121_2
        fn_state.ec = s_121_2;
        // N s_121_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_122_0: const #30u : u32
        let s_122_0: u32 = 30;
        // D s_122_1: read-var exceptype:u32
        let s_122_1: u32 = fn_state.exceptype;
        // D s_122_2: cmp-eq s_122_0 s_122_1
        let s_122_2: bool = ((s_122_0) == (s_122_1));
        // D s_122_3: not s_122_2
        let s_122_3: bool = !s_122_2;
        // N s_122_4: branch s_122_3 b124 b123
        if s_122_3 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_123_0: const #58u : u8
        let s_123_0: u8 = 58;
        // C s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 8u16);
        // C s_123_2: cast zx s_123_1 -> i
        let s_123_2: i128 = (s_123_1.value() as i128);
        // D s_123_3: write-var ec <= s_123_2
        fn_state.ec = s_123_2;
        // C s_123_4: const #0u : u8
        let s_123_4: bool = false;
        // D s_123_5: write-var il_is_valid <= s_123_4
        fn_state.il_is_valid = s_123_4;
        // D s_123_6: read-var from_32:u8
        let s_123_6: bool = fn_state.from_32;
        // N s_123_7: assert s_123_6
        let s_123_7: () = assert!(s_123_6);
        // N s_123_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_124_0: const #() : ()
        let s_124_0: () = ();
        // S s_124_1: call Unreachable(s_124_0)
        let s_124_1: () = Unreachable(state, tracer, s_124_0);
        // N s_124_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}

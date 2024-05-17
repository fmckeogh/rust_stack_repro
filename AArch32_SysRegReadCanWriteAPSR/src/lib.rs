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
use common::*;
pub fn AArch32_SysRegReadCanWriteAPSR<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cp_num: i128,
    instr: u32,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_31641: bool,
        return_value: bool,
        opc1: i64,
        CRn: i64,
        gs_31645: bool,
        gs_31619: bool,
        CRm: i64,
        opc2: i64,
        gs_31639: bool,
        gs_31643: bool,
        cp_num: i128,
        instr: u32,
    }
    let fn_state = FunctionState {
        cp_num,
        instr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call UsingAArch32(s_0_0)
        let s_0_1: bool = UsingAArch32(state, tracer, s_0_0);
        // N s_0_2: assert s_0_1
        let s_0_2: () = assert!(s_0_1);
        // C s_0_3: const #14s : i
        let s_0_3: i128 = 14;
        // D s_0_4: read-var cp_num:i
        let s_0_4: i128 = fn_state.cp_num;
        // D s_0_5: cmp-eq s_0_4 s_0_3
        let s_0_5: bool = ((s_0_4) == (s_0_3));
        // N s_0_6: branch s_0_5 b18 b1
        if s_0_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #15s : i
        let s_1_0: i128 = 15;
        // D s_1_1: read-var cp_num:i
        let s_1_1: i128 = fn_state.cp_num;
        // D s_1_2: cmp-eq s_1_1 s_1_0
        let s_1_2: bool = ((s_1_1) == (s_1_0));
        // D s_1_3: write-var gs#31619 <= s_1_2
        fn_state.gs_31619 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#31619:u8
        let s_2_0: bool = fn_state.gs_31619;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #8s : i
        let s_2_2: i128 = 8;
        // D s_2_3: read-var instr:u32
        let s_2_3: u32 = fn_state.instr;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 32u16);
        // C s_2_5: const #1s : i64
        let s_2_5: i64 = 1;
        // C s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // C s_2_7: const #3s : i
        let s_2_7: i128 = 3;
        // C s_2_8: add s_2_7 s_2_6
        let s_2_8: i128 = (s_2_7 + s_2_6);
        // D s_2_9: bit-extract s_2_4 s_2_2 s_2_8
        let s_2_9: Bits = (Bits::new(
            ((s_2_4) >> (s_2_2)).value(),
            u16::try_from(s_2_8).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u8
        let s_2_10: u8 = (s_2_9.value() as u8);
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 4u16);
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (s_2_11.value() as i128);
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_15: read-var cp_num:i
        let s_2_15: i128 = fn_state.cp_num;
        // D s_2_16: cmp-eq s_2_15 s_2_14
        let s_2_16: bool = ((s_2_15) == (s_2_14));
        // N s_2_17: assert s_2_16
        let s_2_17: () = assert!(s_2_16);
        // C s_2_18: const #21s : i
        let s_2_18: i128 = 21;
        // D s_2_19: read-var instr:u32
        let s_2_19: u32 = fn_state.instr;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 32u16);
        // C s_2_21: const #1s : i64
        let s_2_21: i64 = 1;
        // C s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // C s_2_23: const #2s : i
        let s_2_23: i128 = 2;
        // C s_2_24: add s_2_23 s_2_22
        let s_2_24: i128 = (s_2_23 + s_2_22);
        // D s_2_25: bit-extract s_2_20 s_2_18 s_2_24
        let s_2_25: Bits = (Bits::new(
            ((s_2_20) >> (s_2_18)).value(),
            u16::try_from(s_2_24).unwrap(),
        ));
        // D s_2_26: cast reint s_2_25 -> u8
        let s_2_26: u8 = (s_2_25.value() as u8);
        // D s_2_27: cast zx s_2_26 -> bv
        let s_2_27: Bits = Bits::new(s_2_26 as u128, 3u16);
        // D s_2_28: cast zx s_2_27 -> i
        let s_2_28: i128 = (s_2_27.value() as i128);
        // D s_2_29: cast reint s_2_28 -> i64
        let s_2_29: i64 = (s_2_28 as i64);
        // D s_2_30: write-var opc1 <= s_2_29
        fn_state.opc1 = s_2_29;
        // C s_2_31: const #5s : i
        let s_2_31: i128 = 5;
        // D s_2_32: read-var instr:u32
        let s_2_32: u32 = fn_state.instr;
        // D s_2_33: cast zx s_2_32 -> bv
        let s_2_33: Bits = Bits::new(s_2_32 as u128, 32u16);
        // C s_2_34: const #1s : i64
        let s_2_34: i64 = 1;
        // C s_2_35: cast zx s_2_34 -> i
        let s_2_35: i128 = (i128::try_from(s_2_34).unwrap());
        // C s_2_36: const #2s : i
        let s_2_36: i128 = 2;
        // C s_2_37: add s_2_36 s_2_35
        let s_2_37: i128 = (s_2_36 + s_2_35);
        // D s_2_38: bit-extract s_2_33 s_2_31 s_2_37
        let s_2_38: Bits = (Bits::new(
            ((s_2_33) >> (s_2_31)).value(),
            u16::try_from(s_2_37).unwrap(),
        ));
        // D s_2_39: cast reint s_2_38 -> u8
        let s_2_39: u8 = (s_2_38.value() as u8);
        // D s_2_40: cast zx s_2_39 -> bv
        let s_2_40: Bits = Bits::new(s_2_39 as u128, 3u16);
        // D s_2_41: cast zx s_2_40 -> i
        let s_2_41: i128 = (s_2_40.value() as i128);
        // D s_2_42: cast reint s_2_41 -> i64
        let s_2_42: i64 = (s_2_41 as i64);
        // D s_2_43: write-var opc2 <= s_2_42
        fn_state.opc2 = s_2_42;
        // C s_2_44: const #16s : i
        let s_2_44: i128 = 16;
        // D s_2_45: read-var instr:u32
        let s_2_45: u32 = fn_state.instr;
        // D s_2_46: cast zx s_2_45 -> bv
        let s_2_46: Bits = Bits::new(s_2_45 as u128, 32u16);
        // C s_2_47: const #1s : i64
        let s_2_47: i64 = 1;
        // C s_2_48: cast zx s_2_47 -> i
        let s_2_48: i128 = (i128::try_from(s_2_47).unwrap());
        // C s_2_49: const #3s : i
        let s_2_49: i128 = 3;
        // C s_2_50: add s_2_49 s_2_48
        let s_2_50: i128 = (s_2_49 + s_2_48);
        // D s_2_51: bit-extract s_2_46 s_2_44 s_2_50
        let s_2_51: Bits = (Bits::new(
            ((s_2_46) >> (s_2_44)).value(),
            u16::try_from(s_2_50).unwrap(),
        ));
        // D s_2_52: cast reint s_2_51 -> u8
        let s_2_52: u8 = (s_2_51.value() as u8);
        // D s_2_53: cast zx s_2_52 -> bv
        let s_2_53: Bits = Bits::new(s_2_52 as u128, 4u16);
        // D s_2_54: cast zx s_2_53 -> i
        let s_2_54: i128 = (s_2_53.value() as i128);
        // D s_2_55: cast reint s_2_54 -> i64
        let s_2_55: i64 = (s_2_54 as i64);
        // D s_2_56: write-var CRn <= s_2_55
        fn_state.CRn = s_2_55;
        // C s_2_57: const #0s : i
        let s_2_57: i128 = 0;
        // D s_2_58: read-var instr:u32
        let s_2_58: u32 = fn_state.instr;
        // D s_2_59: cast zx s_2_58 -> bv
        let s_2_59: Bits = Bits::new(s_2_58 as u128, 32u16);
        // C s_2_60: const #1s : i64
        let s_2_60: i64 = 1;
        // C s_2_61: cast zx s_2_60 -> i
        let s_2_61: i128 = (i128::try_from(s_2_60).unwrap());
        // C s_2_62: const #3s : i
        let s_2_62: i128 = 3;
        // C s_2_63: add s_2_62 s_2_61
        let s_2_63: i128 = (s_2_62 + s_2_61);
        // D s_2_64: bit-extract s_2_59 s_2_57 s_2_63
        let s_2_64: Bits = (Bits::new(
            ((s_2_59) >> (s_2_57)).value(),
            u16::try_from(s_2_63).unwrap(),
        ));
        // D s_2_65: cast reint s_2_64 -> u8
        let s_2_65: u8 = (s_2_64.value() as u8);
        // D s_2_66: cast zx s_2_65 -> bv
        let s_2_66: Bits = Bits::new(s_2_65 as u128, 4u16);
        // D s_2_67: cast zx s_2_66 -> i
        let s_2_67: i128 = (s_2_66.value() as i128);
        // D s_2_68: cast reint s_2_67 -> i64
        let s_2_68: i64 = (s_2_67 as i64);
        // D s_2_69: write-var CRm <= s_2_68
        fn_state.CRm = s_2_68;
        // C s_2_70: const #14s : i
        let s_2_70: i128 = 14;
        // D s_2_71: read-var cp_num:i
        let s_2_71: i128 = fn_state.cp_num;
        // D s_2_72: cmp-eq s_2_71 s_2_70
        let s_2_72: bool = ((s_2_71) == (s_2_70));
        // N s_2_73: branch s_2_72 b17 b3
        if s_2_72 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#31639 <= s_3_0
        fn_state.gs_31639 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#31639:u8
        let s_4_0: bool = fn_state.gs_31639;
        // N s_4_1: branch s_4_0 b16 b5
        if s_4_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#31641 <= s_5_0
        fn_state.gs_31641 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#31641:u8
        let s_6_0: bool = fn_state.gs_31641;
        // N s_6_1: branch s_6_0 b15 b7
        if s_6_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#31643 <= s_7_0
        fn_state.gs_31643 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#31643:u8
        let s_8_0: bool = fn_state.gs_31643;
        // N s_8_1: branch s_8_0 b14 b9
        if s_8_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#31645 <= s_9_0
        fn_state.gs_31645 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#31645:u8
        let s_10_0: bool = fn_state.gs_31645;
        // N s_10_1: branch s_10_0 b13 b11
        if s_10_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
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
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var return_value:u8
        let s_12_0: bool = fn_state.return_value;
        // N s_12_1: return s_12_0
        return s_12_0;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var return_value <= s_13_0
        fn_state.return_value = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var opc2:i64
        let s_14_1: i64 = fn_state.opc2;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_0
        let s_14_3: bool = ((s_14_2) == (s_14_0));
        // D s_14_4: write-var gs#31645 <= s_14_3
        fn_state.gs_31645 = s_14_3;
        // N s_14_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #1s : i
        let s_15_0: i128 = 1;
        // D s_15_1: read-var CRm:i64
        let s_15_1: i64 = fn_state.CRm;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: cmp-eq s_15_2 s_15_0
        let s_15_3: bool = ((s_15_2) == (s_15_0));
        // D s_15_4: write-var gs#31643 <= s_15_3
        fn_state.gs_31643 = s_15_3;
        // N s_15_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #0s : i
        let s_16_0: i128 = 0;
        // D s_16_1: read-var CRn:i64
        let s_16_1: i64 = fn_state.CRn;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: cmp-eq s_16_2 s_16_0
        let s_16_3: bool = ((s_16_2) == (s_16_0));
        // D s_16_4: write-var gs#31641 <= s_16_3
        fn_state.gs_31641 = s_16_3;
        // N s_16_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0s : i
        let s_17_0: i128 = 0;
        // D s_17_1: read-var opc1:i64
        let s_17_1: i64 = fn_state.opc1;
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: cmp-eq s_17_2 s_17_0
        let s_17_3: bool = ((s_17_2) == (s_17_0));
        // D s_17_4: write-var gs#31639 <= s_17_3
        fn_state.gs_31639 = s_17_3;
        // N s_17_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#31619 <= s_18_0
        fn_state.gs_31619 = s_18_0;
        // N s_18_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}

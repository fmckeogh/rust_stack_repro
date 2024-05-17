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
use VTTBR_EL2_SysRegWrite128_3ec09d9d33270957::*;
use TTBR1_EL2_SysRegWrite128_4bfd1913f2c26b7d::*;
use S3_op1_Cn_Cm_op2_SysRegWrite128_54572e14feed5fe8::*;
use RCWMASK_EL1_SysRegWrite128_b8a59eaf95f35a5f::*;
use PAR_EL1_SysRegWrite128_e67f3244d494abe6::*;
use TTBR1_EL2_SysRegWrite128_c87556637100112b::*;
use TTBR0_EL1_SysRegWrite128_66ee61de597f133e::*;
use RCWSMASK_EL1_SysRegWrite128_d70b97b0479af313::*;
use TTBR0_EL1_SysRegWrite128_8ea27f288a4b152e::*;
use TTBR0_EL2_SysRegWrite128_db720477dd35bd78::*;
use TTBR1_EL1_SysRegWrite128_d20c29029eb4ed94::*;
use AArch64_UnallocatedSysRegAccess::*;
use common::*;
pub fn AArch64_AutoGen_SysRegWrite128<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_138030: bool,
        gs_138041: bool,
        gs_138014: bool,
        gs_138029: bool,
        gs_138022: bool,
        gs_138064: bool,
        gs_138035: bool,
        gs_138026: bool,
        gs_138037: bool,
        gs_138028: bool,
        gs_138025: bool,
        gs_138031: bool,
        gs_138050: bool,
        gs_138054: bool,
        gs_138012: bool,
        gs_138019: bool,
        gs_138063: bool,
        gs_138032: bool,
        gs_138018: bool,
        gs_138016: bool,
        gs_138038: bool,
        gs_138070: bool,
        gs_138023: bool,
        gs_138011: bool,
        gs_138058: bool,
        gs_138062: bool,
        gs_138039: bool,
        gs_138045: bool,
        gs_138036: bool,
        gs_138066: bool,
        gs_138033: bool,
        gs_138024: bool,
        gs_138027: bool,
        gs_138013: bool,
        gs_138021: bool,
        b__1: u8,
        gs_138034: bool,
        gs_138040: bool,
        gs_138067: bool,
        gs_138015: bool,
        gs_138068: bool,
        gs_138010: bool,
        gs_138017: bool,
        gs_138069: bool,
        gs_138020: bool,
        gs_138065: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        t2,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var CRm:u8
        let s_0_0: u8 = fn_state.CRm;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // C s_0_2: const #4u : u8
        let s_0_2: u8 = 4;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 4u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b157 b1
        if s_0_4 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#138010 <= s_1_0
        fn_state.gs_138010 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#138010:u8
        let s_2_0: bool = fn_state.gs_138010;
        // N s_2_1: branch s_2_0 b156 b3
        if s_2_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#138011 <= s_3_0
        fn_state.gs_138011 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#138011:u8
        let s_4_0: bool = fn_state.gs_138011;
        // N s_4_1: branch s_4_0 b155 b5
        if s_4_0 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#138012 <= s_5_0
        fn_state.gs_138012 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#138012:u8
        let s_6_0: bool = fn_state.gs_138012;
        // N s_6_1: branch s_6_0 b154 b7
        if s_6_0 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#138013 <= s_7_0
        fn_state.gs_138013 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#138013:u8
        let s_8_0: bool = fn_state.gs_138013;
        // N s_8_1: branch s_8_0 b153 b9
        if s_8_0 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var CRm:u8
        let s_9_0: u8 = fn_state.CRm;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 4u16);
        // C s_9_2: const #0u : u8
        let s_9_2: u8 = 0;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 4u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b152 b10
        if s_9_4 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#138014 <= s_10_0
        fn_state.gs_138014 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#138014:u8
        let s_11_0: bool = fn_state.gs_138014;
        // N s_11_1: branch s_11_0 b151 b12
        if s_11_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#138015 <= s_12_0
        fn_state.gs_138015 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#138015:u8
        let s_13_0: bool = fn_state.gs_138015;
        // N s_13_1: branch s_13_0 b150 b14
        if s_13_0 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#138016 <= s_14_0
        fn_state.gs_138016 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#138016:u8
        let s_15_0: bool = fn_state.gs_138016;
        // N s_15_1: branch s_15_0 b149 b16
        if s_15_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#138017 <= s_16_0
        fn_state.gs_138017 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#138017:u8
        let s_17_0: bool = fn_state.gs_138017;
        // N s_17_1: branch s_17_0 b148 b18
        if s_17_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var CRm:u8
        let s_18_0: u8 = fn_state.CRm;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 4u16);
        // C s_18_2: const #0u : u8
        let s_18_2: u8 = 0;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 4u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b147 b19
        if s_18_4 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#138018 <= s_19_0
        fn_state.gs_138018 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#138018:u8
        let s_20_0: bool = fn_state.gs_138018;
        // N s_20_1: branch s_20_0 b146 b21
        if s_20_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#138019 <= s_21_0
        fn_state.gs_138019 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#138019:u8
        let s_22_0: bool = fn_state.gs_138019;
        // N s_22_1: branch s_22_0 b145 b23
        if s_22_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#138020 <= s_23_0
        fn_state.gs_138020 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#138020:u8
        let s_24_0: bool = fn_state.gs_138020;
        // N s_24_1: branch s_24_0 b144 b25
        if s_24_0 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#138021 <= s_25_0
        fn_state.gs_138021 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#138021:u8
        let s_26_0: bool = fn_state.gs_138021;
        // N s_26_1: branch s_26_0 b143 b27
        if s_26_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var CRm:u8
        let s_27_0: u8 = fn_state.CRm;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 4u16);
        // C s_27_2: const #0u : u8
        let s_27_2: u8 = 0;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 4u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // N s_27_5: branch s_27_4 b142 b28
        if s_27_4 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#138022 <= s_28_0
        fn_state.gs_138022 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#138022:u8
        let s_29_0: bool = fn_state.gs_138022;
        // N s_29_1: branch s_29_0 b141 b30
        if s_29_0 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#138023 <= s_30_0
        fn_state.gs_138023 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#138023:u8
        let s_31_0: bool = fn_state.gs_138023;
        // N s_31_1: branch s_31_0 b140 b32
        if s_31_0 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#138024 <= s_32_0
        fn_state.gs_138024 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#138024:u8
        let s_33_0: bool = fn_state.gs_138024;
        // N s_33_1: branch s_33_0 b139 b34
        if s_33_0 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#138025 <= s_34_0
        fn_state.gs_138025 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#138025:u8
        let s_35_0: bool = fn_state.gs_138025;
        // N s_35_1: branch s_35_0 b138 b36
        if s_35_0 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var CRm:u8
        let s_36_0: u8 = fn_state.CRm;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 4u16);
        // C s_36_2: const #0u : u8
        let s_36_2: u8 = 0;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 4u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // N s_36_5: branch s_36_4 b137 b37
        if s_36_4 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#138026 <= s_37_0
        fn_state.gs_138026 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#138026:u8
        let s_38_0: bool = fn_state.gs_138026;
        // N s_38_1: branch s_38_0 b136 b39
        if s_38_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#138027 <= s_39_0
        fn_state.gs_138027 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#138027:u8
        let s_40_0: bool = fn_state.gs_138027;
        // N s_40_1: branch s_40_0 b135 b41
        if s_40_0 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#138028 <= s_41_0
        fn_state.gs_138028 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#138028:u8
        let s_42_0: bool = fn_state.gs_138028;
        // N s_42_1: branch s_42_0 b134 b43
        if s_42_0 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#138029 <= s_43_0
        fn_state.gs_138029 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#138029:u8
        let s_44_0: bool = fn_state.gs_138029;
        // N s_44_1: branch s_44_0 b133 b45
        if s_44_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var CRm:u8
        let s_45_0: u8 = fn_state.CRm;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 4u16);
        // C s_45_2: const #0u : u8
        let s_45_2: u8 = 0;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 4u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // N s_45_5: branch s_45_4 b132 b46
        if s_45_4 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#138030 <= s_46_0
        fn_state.gs_138030 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#138030:u8
        let s_47_0: bool = fn_state.gs_138030;
        // N s_47_1: branch s_47_0 b131 b48
        if s_47_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#138031 <= s_48_0
        fn_state.gs_138031 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#138031:u8
        let s_49_0: bool = fn_state.gs_138031;
        // N s_49_1: branch s_49_0 b130 b50
        if s_49_0 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#138032 <= s_50_0
        fn_state.gs_138032 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#138032:u8
        let s_51_0: bool = fn_state.gs_138032;
        // N s_51_1: branch s_51_0 b129 b52
        if s_51_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#138033 <= s_52_0
        fn_state.gs_138033 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#138033:u8
        let s_53_0: bool = fn_state.gs_138033;
        // N s_53_1: branch s_53_0 b128 b54
        if s_53_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var CRm:u8
        let s_54_0: u8 = fn_state.CRm;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 4u16);
        // C s_54_2: const #1u : u8
        let s_54_2: u8 = 1;
        // C s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 4u16);
        // D s_54_4: cmp-eq s_54_1 s_54_3
        let s_54_4: bool = ((s_54_1) == (s_54_3));
        // N s_54_5: branch s_54_4 b127 b55
        if s_54_4 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#138034 <= s_55_0
        fn_state.gs_138034 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#138034:u8
        let s_56_0: bool = fn_state.gs_138034;
        // N s_56_1: branch s_56_0 b126 b57
        if s_56_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#138035 <= s_57_0
        fn_state.gs_138035 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#138035:u8
        let s_58_0: bool = fn_state.gs_138035;
        // N s_58_1: branch s_58_0 b125 b59
        if s_58_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#138036 <= s_59_0
        fn_state.gs_138036 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#138036:u8
        let s_60_0: bool = fn_state.gs_138036;
        // N s_60_1: branch s_60_0 b124 b61
        if s_60_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#138037 <= s_61_0
        fn_state.gs_138037 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#138037:u8
        let s_62_0: bool = fn_state.gs_138037;
        // N s_62_1: branch s_62_0 b123 b63
        if s_62_0 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var CRm:u8
        let s_63_0: u8 = fn_state.CRm;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 4u16);
        // C s_63_2: const #0u : u8
        let s_63_2: u8 = 0;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 4u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // N s_63_5: branch s_63_4 b122 b64
        if s_63_4 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var gs#138038 <= s_64_0
        fn_state.gs_138038 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#138038:u8
        let s_65_0: bool = fn_state.gs_138038;
        // N s_65_1: branch s_65_0 b121 b66
        if s_65_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#138039 <= s_66_0
        fn_state.gs_138039 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#138039:u8
        let s_67_0: bool = fn_state.gs_138039;
        // N s_67_1: branch s_67_0 b120 b68
        if s_67_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#138040 <= s_68_0
        fn_state.gs_138040 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#138040:u8
        let s_69_0: bool = fn_state.gs_138040;
        // N s_69_1: branch s_69_0 b119 b70
        if s_69_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#138041 <= s_70_0
        fn_state.gs_138041 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#138041:u8
        let s_71_0: bool = fn_state.gs_138041;
        // N s_71_1: branch s_71_0 b118 b72
        if s_71_0 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var CRn:u8
        let s_72_0: u8 = fn_state.CRn;
        // D s_72_1: write-var b__1 <= s_72_0
        fn_state.b__1 = s_72_0;
        // C s_72_2: const #3s : i
        let s_72_2: i128 = 3;
        // D s_72_3: read-var b__1:u8
        let s_72_3: u8 = fn_state.b__1;
        // D s_72_4: cast zx s_72_3 -> bv
        let s_72_4: Bits = Bits::new(s_72_3 as u128, 4u16);
        // C s_72_5: const #1s : i64
        let s_72_5: i64 = 1;
        // C s_72_6: cast zx s_72_5 -> i
        let s_72_6: i128 = (i128::try_from(s_72_5).unwrap());
        // C s_72_7: const #0s : i
        let s_72_7: i128 = 0;
        // C s_72_8: add s_72_7 s_72_6
        let s_72_8: i128 = (s_72_7 + s_72_6);
        // D s_72_9: bit-extract s_72_4 s_72_2 s_72_8
        let s_72_9: Bits = (Bits::new(
            ((s_72_4) >> (s_72_2)).value(),
            u16::try_from(s_72_8).unwrap(),
        ));
        // D s_72_10: cast reint s_72_9 -> u8
        let s_72_10: bool = ((s_72_9.value()) != 0);
        // D s_72_11: cast zx s_72_10 -> bv
        let s_72_11: Bits = Bits::new(s_72_10 as u128, 1u16);
        // C s_72_12: const #1u : u8
        let s_72_12: bool = true;
        // C s_72_13: cast zx s_72_12 -> bv
        let s_72_13: Bits = Bits::new(s_72_12 as u128, 1u16);
        // D s_72_14: cmp-eq s_72_11 s_72_13
        let s_72_14: bool = ((s_72_11) == (s_72_13));
        // N s_72_15: branch s_72_14 b117 b73
        if s_72_14 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#138050 <= s_73_0
        fn_state.gs_138050 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#138050:u8
        let s_74_0: bool = fn_state.gs_138050;
        // D s_74_1: not s_74_0
        let s_74_1: bool = !s_74_0;
        // N s_74_2: branch s_74_1 b116 b75
        if s_74_1 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#138045 <= s_75_0
        fn_state.gs_138045 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#138045:u8
        let s_76_0: bool = fn_state.gs_138045;
        // N s_76_1: branch s_76_0 b115 b77
        if s_76_0 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#138054 <= s_77_0
        fn_state.gs_138054 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#138054:u8
        let s_78_0: bool = fn_state.gs_138054;
        // N s_78_1: branch s_78_0 b114 b79
        if s_78_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#138058 <= s_79_0
        fn_state.gs_138058 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#138058:u8
        let s_80_0: bool = fn_state.gs_138058;
        // N s_80_1: branch s_80_0 b113 b81
        if s_80_0 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#138062 <= s_81_0
        fn_state.gs_138062 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#138062:u8
        let s_82_0: bool = fn_state.gs_138062;
        // N s_82_1: branch s_82_0 b112 b83
        if s_82_0 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var CRm:u8
        let s_83_0: u8 = fn_state.CRm;
        // D s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 4u16);
        // C s_83_2: const #0u : u8
        let s_83_2: u8 = 0;
        // C s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 4u16);
        // D s_83_4: cmp-eq s_83_1 s_83_3
        let s_83_4: bool = ((s_83_1) == (s_83_3));
        // N s_83_5: branch s_83_4 b111 b84
        if s_83_4 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var gs#138063 <= s_84_0
        fn_state.gs_138063 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#138063:u8
        let s_85_0: bool = fn_state.gs_138063;
        // N s_85_1: branch s_85_0 b110 b86
        if s_85_0 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#138064 <= s_86_0
        fn_state.gs_138064 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#138064:u8
        let s_87_0: bool = fn_state.gs_138064;
        // N s_87_1: branch s_87_0 b109 b88
        if s_87_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#138065 <= s_88_0
        fn_state.gs_138065 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#138065:u8
        let s_89_0: bool = fn_state.gs_138065;
        // N s_89_1: branch s_89_0 b108 b90
        if s_89_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#138066 <= s_90_0
        fn_state.gs_138066 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#138066:u8
        let s_91_0: bool = fn_state.gs_138066;
        // N s_91_1: branch s_91_0 b107 b92
        if s_91_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var CRm:u8
        let s_92_0: u8 = fn_state.CRm;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 4u16);
        // C s_92_2: const #0u : u8
        let s_92_2: u8 = 0;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 4u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // N s_92_5: branch s_92_4 b106 b93
        if s_92_4 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#138067 <= s_93_0
        fn_state.gs_138067 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#138067:u8
        let s_94_0: bool = fn_state.gs_138067;
        // N s_94_1: branch s_94_0 b105 b95
        if s_94_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // D s_95_1: write-var gs#138068 <= s_95_0
        fn_state.gs_138068 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#138068:u8
        let s_96_0: bool = fn_state.gs_138068;
        // N s_96_1: branch s_96_0 b104 b97
        if s_96_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#138069 <= s_97_0
        fn_state.gs_138069 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#138069:u8
        let s_98_0: bool = fn_state.gs_138069;
        // N s_98_1: branch s_98_0 b103 b99
        if s_98_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#138070 <= s_99_0
        fn_state.gs_138070 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#138070:u8
        let s_100_0: bool = fn_state.gs_138070;
        // N s_100_1: branch s_100_0 b102 b101
        if s_100_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var op0:u8
        let s_101_0: u8 = fn_state.op0;
        // D s_101_1: read-var op1:u8
        let s_101_1: u8 = fn_state.op1;
        // D s_101_2: read-var CRn:u8
        let s_101_2: u8 = fn_state.CRn;
        // D s_101_3: read-var op2:u8
        let s_101_3: u8 = fn_state.op2;
        // D s_101_4: read-var CRm:u8
        let s_101_4: u8 = fn_state.CRm;
        // C s_101_5: const #0u : u8
        let s_101_5: bool = false;
        // D s_101_6: read-var t:i
        let s_101_6: i128 = fn_state.t;
        // D s_101_7: call AArch64_UnallocatedSysRegAccess(s_101_0, s_101_1, s_101_2, s_101_3, s_101_4, s_101_5, s_101_6)
        let s_101_7: () = AArch64_UnallocatedSysRegAccess(
            state,
            tracer,
            s_101_0,
            s_101_1,
            s_101_2,
            s_101_3,
            s_101_4,
            s_101_5,
            s_101_6,
        );
        // N s_101_8: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var el:u8
        let s_102_0: u8 = fn_state.el;
        // D s_102_1: read-var op0:u8
        let s_102_1: u8 = fn_state.op0;
        // D s_102_2: read-var op1:u8
        let s_102_2: u8 = fn_state.op1;
        // D s_102_3: read-var CRn:u8
        let s_102_3: u8 = fn_state.CRn;
        // D s_102_4: read-var op2:u8
        let s_102_4: u8 = fn_state.op2;
        // D s_102_5: read-var CRm:u8
        let s_102_5: u8 = fn_state.CRm;
        // D s_102_6: read-var t:i
        let s_102_6: i128 = fn_state.t;
        // D s_102_7: read-var t2:i
        let s_102_7: i128 = fn_state.t2;
        // D s_102_8: call TTBR1_EL2_SysRegWrite128_c87556637100112b(s_102_0, s_102_1, s_102_2, s_102_3, s_102_4, s_102_5, s_102_6, s_102_7)
        let s_102_8: () = TTBR1_EL2_SysRegWrite128_c87556637100112b(
            state,
            tracer,
            s_102_0,
            s_102_1,
            s_102_2,
            s_102_3,
            s_102_4,
            s_102_5,
            s_102_6,
            s_102_7,
        );
        // N s_102_9: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var op2:u8
        let s_103_0: u8 = fn_state.op2;
        // D s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 3u16);
        // C s_103_2: const #1u : u8
        let s_103_2: u8 = 1;
        // C s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 3u16);
        // D s_103_4: cmp-eq s_103_1 s_103_3
        let s_103_4: bool = ((s_103_1) == (s_103_3));
        // D s_103_5: write-var gs#138070 <= s_103_4
        fn_state.gs_138070 = s_103_4;
        // N s_103_6: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var op1:u8
        let s_104_0: u8 = fn_state.op1;
        // D s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 3u16);
        // C s_104_2: const #4u : u8
        let s_104_2: u8 = 4;
        // C s_104_3: cast zx s_104_2 -> bv
        let s_104_3: Bits = Bits::new(s_104_2 as u128, 3u16);
        // D s_104_4: cmp-eq s_104_1 s_104_3
        let s_104_4: bool = ((s_104_1) == (s_104_3));
        // D s_104_5: write-var gs#138069 <= s_104_4
        fn_state.gs_138069 = s_104_4;
        // N s_104_6: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var op0:u8
        let s_105_0: u8 = fn_state.op0;
        // D s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 2u16);
        // C s_105_2: const #3u : u8
        let s_105_2: u8 = 3;
        // C s_105_3: cast zx s_105_2 -> bv
        let s_105_3: Bits = Bits::new(s_105_2 as u128, 2u16);
        // D s_105_4: cmp-eq s_105_1 s_105_3
        let s_105_4: bool = ((s_105_1) == (s_105_3));
        // D s_105_5: write-var gs#138068 <= s_105_4
        fn_state.gs_138068 = s_105_4;
        // N s_105_6: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var CRn:u8
        let s_106_0: u8 = fn_state.CRn;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 4u16);
        // C s_106_2: const #2u : u8
        let s_106_2: u8 = 2;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 4u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#138067 <= s_106_4
        fn_state.gs_138067 = s_106_4;
        // N s_106_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var el:u8
        let s_107_0: u8 = fn_state.el;
        // D s_107_1: read-var op0:u8
        let s_107_1: u8 = fn_state.op0;
        // D s_107_2: read-var op1:u8
        let s_107_2: u8 = fn_state.op1;
        // D s_107_3: read-var CRn:u8
        let s_107_3: u8 = fn_state.CRn;
        // D s_107_4: read-var op2:u8
        let s_107_4: u8 = fn_state.op2;
        // D s_107_5: read-var CRm:u8
        let s_107_5: u8 = fn_state.CRm;
        // D s_107_6: read-var t:i
        let s_107_6: i128 = fn_state.t;
        // D s_107_7: read-var t2:i
        let s_107_7: i128 = fn_state.t2;
        // D s_107_8: call RCWMASK_EL1_SysRegWrite128_b8a59eaf95f35a5f(s_107_0, s_107_1, s_107_2, s_107_3, s_107_4, s_107_5, s_107_6, s_107_7)
        let s_107_8: () = RCWMASK_EL1_SysRegWrite128_b8a59eaf95f35a5f(
            state,
            tracer,
            s_107_0,
            s_107_1,
            s_107_2,
            s_107_3,
            s_107_4,
            s_107_5,
            s_107_6,
            s_107_7,
        );
        // N s_107_9: return
        return;
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var op2:u8
        let s_108_0: u8 = fn_state.op2;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 3u16);
        // C s_108_2: const #6u : u8
        let s_108_2: u8 = 6;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 3u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#138066 <= s_108_4
        fn_state.gs_138066 = s_108_4;
        // N s_108_6: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var op1:u8
        let s_109_0: u8 = fn_state.op1;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 3u16);
        // C s_109_2: const #0u : u8
        let s_109_2: u8 = 0;
        // C s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 3u16);
        // D s_109_4: cmp-eq s_109_1 s_109_3
        let s_109_4: bool = ((s_109_1) == (s_109_3));
        // D s_109_5: write-var gs#138065 <= s_109_4
        fn_state.gs_138065 = s_109_4;
        // N s_109_6: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var op0:u8
        let s_110_0: u8 = fn_state.op0;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 2u16);
        // C s_110_2: const #3u : u8
        let s_110_2: u8 = 3;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 2u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // D s_110_5: write-var gs#138064 <= s_110_4
        fn_state.gs_138064 = s_110_4;
        // N s_110_6: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var CRn:u8
        let s_111_0: u8 = fn_state.CRn;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 4u16);
        // C s_111_2: const #13u : u8
        let s_111_2: u8 = 13;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 4u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // D s_111_5: write-var gs#138063 <= s_111_4
        fn_state.gs_138063 = s_111_4;
        // N s_111_6: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var el:u8
        let s_112_0: u8 = fn_state.el;
        // D s_112_1: read-var op0:u8
        let s_112_1: u8 = fn_state.op0;
        // D s_112_2: read-var op1:u8
        let s_112_2: u8 = fn_state.op1;
        // D s_112_3: read-var CRn:u8
        let s_112_3: u8 = fn_state.CRn;
        // D s_112_4: read-var op2:u8
        let s_112_4: u8 = fn_state.op2;
        // D s_112_5: read-var CRm:u8
        let s_112_5: u8 = fn_state.CRm;
        // D s_112_6: read-var t:i
        let s_112_6: i128 = fn_state.t;
        // D s_112_7: read-var t2:i
        let s_112_7: i128 = fn_state.t2;
        // D s_112_8: call S3_op1_Cn_Cm_op2_SysRegWrite128_54572e14feed5fe8(s_112_0, s_112_1, s_112_2, s_112_3, s_112_4, s_112_5, s_112_6, s_112_7)
        let s_112_8: () = S3_op1_Cn_Cm_op2_SysRegWrite128_54572e14feed5fe8(
            state,
            tracer,
            s_112_0,
            s_112_1,
            s_112_2,
            s_112_3,
            s_112_4,
            s_112_5,
            s_112_6,
            s_112_7,
        );
        // N s_112_9: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #1u : u8
        let s_113_0: bool = true;
        // D s_113_1: write-var gs#138062 <= s_113_0
        fn_state.gs_138062 = s_113_0;
        // N s_113_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // D s_114_1: write-var gs#138058 <= s_114_0
        fn_state.gs_138058 = s_114_0;
        // N s_114_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var op0:u8
        let s_115_0: u8 = fn_state.op0;
        // D s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 2u16);
        // C s_115_2: const #3u : u8
        let s_115_2: u8 = 3;
        // C s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 2u16);
        // D s_115_4: cmp-eq s_115_1 s_115_3
        let s_115_4: bool = ((s_115_1) == (s_115_3));
        // D s_115_5: write-var gs#138054 <= s_115_4
        fn_state.gs_138054 = s_115_4;
        // N s_115_6: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #0u : u8
        let s_116_0: bool = false;
        // D s_116_1: write-var gs#138045 <= s_116_0
        fn_state.gs_138045 = s_116_0;
        // N s_116_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #0s : i
        let s_117_0: i128 = 0;
        // D s_117_1: read-var b__1:u8
        let s_117_1: u8 = fn_state.b__1;
        // D s_117_2: cast zx s_117_1 -> bv
        let s_117_2: Bits = Bits::new(s_117_1 as u128, 4u16);
        // C s_117_3: const #1s : i64
        let s_117_3: i64 = 1;
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // C s_117_5: const #1s : i
        let s_117_5: i128 = 1;
        // C s_117_6: add s_117_5 s_117_4
        let s_117_6: i128 = (s_117_5 + s_117_4);
        // D s_117_7: bit-extract s_117_2 s_117_0 s_117_6
        let s_117_7: Bits = (Bits::new(
            ((s_117_2) >> (s_117_0)).value(),
            u16::try_from(s_117_6).unwrap(),
        ));
        // D s_117_8: cast reint s_117_7 -> u8
        let s_117_8: u8 = (s_117_7.value() as u8);
        // D s_117_9: cast zx s_117_8 -> bv
        let s_117_9: Bits = Bits::new(s_117_8 as u128, 2u16);
        // C s_117_10: const #3u : u8
        let s_117_10: u8 = 3;
        // C s_117_11: cast zx s_117_10 -> bv
        let s_117_11: Bits = Bits::new(s_117_10 as u128, 2u16);
        // D s_117_12: cmp-eq s_117_9 s_117_11
        let s_117_12: bool = ((s_117_9) == (s_117_11));
        // D s_117_13: write-var gs#138050 <= s_117_12
        fn_state.gs_138050 = s_117_12;
        // N s_117_14: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var el:u8
        let s_118_0: u8 = fn_state.el;
        // D s_118_1: read-var op0:u8
        let s_118_1: u8 = fn_state.op0;
        // D s_118_2: read-var op1:u8
        let s_118_2: u8 = fn_state.op1;
        // D s_118_3: read-var CRn:u8
        let s_118_3: u8 = fn_state.CRn;
        // D s_118_4: read-var op2:u8
        let s_118_4: u8 = fn_state.op2;
        // D s_118_5: read-var CRm:u8
        let s_118_5: u8 = fn_state.CRm;
        // D s_118_6: read-var t:i
        let s_118_6: i128 = fn_state.t;
        // D s_118_7: read-var t2:i
        let s_118_7: i128 = fn_state.t2;
        // D s_118_8: call TTBR0_EL1_SysRegWrite128_8ea27f288a4b152e(s_118_0, s_118_1, s_118_2, s_118_3, s_118_4, s_118_5, s_118_6, s_118_7)
        let s_118_8: () = TTBR0_EL1_SysRegWrite128_8ea27f288a4b152e(
            state,
            tracer,
            s_118_0,
            s_118_1,
            s_118_2,
            s_118_3,
            s_118_4,
            s_118_5,
            s_118_6,
            s_118_7,
        );
        // N s_118_9: return
        return;
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var op2:u8
        let s_119_0: u8 = fn_state.op2;
        // D s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 3u16);
        // C s_119_2: const #0u : u8
        let s_119_2: u8 = 0;
        // C s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 3u16);
        // D s_119_4: cmp-eq s_119_1 s_119_3
        let s_119_4: bool = ((s_119_1) == (s_119_3));
        // D s_119_5: write-var gs#138041 <= s_119_4
        fn_state.gs_138041 = s_119_4;
        // N s_119_6: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var op1:u8
        let s_120_0: u8 = fn_state.op1;
        // D s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 3u16);
        // C s_120_2: const #5u : u8
        let s_120_2: u8 = 5;
        // C s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 3u16);
        // D s_120_4: cmp-eq s_120_1 s_120_3
        let s_120_4: bool = ((s_120_1) == (s_120_3));
        // D s_120_5: write-var gs#138040 <= s_120_4
        fn_state.gs_138040 = s_120_4;
        // N s_120_6: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var op0:u8
        let s_121_0: u8 = fn_state.op0;
        // D s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 2u16);
        // C s_121_2: const #3u : u8
        let s_121_2: u8 = 3;
        // C s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 2u16);
        // D s_121_4: cmp-eq s_121_1 s_121_3
        let s_121_4: bool = ((s_121_1) == (s_121_3));
        // D s_121_5: write-var gs#138039 <= s_121_4
        fn_state.gs_138039 = s_121_4;
        // N s_121_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var CRn:u8
        let s_122_0: u8 = fn_state.CRn;
        // D s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 4u16);
        // C s_122_2: const #2u : u8
        let s_122_2: u8 = 2;
        // C s_122_3: cast zx s_122_2 -> bv
        let s_122_3: Bits = Bits::new(s_122_2 as u128, 4u16);
        // D s_122_4: cmp-eq s_122_1 s_122_3
        let s_122_4: bool = ((s_122_1) == (s_122_3));
        // D s_122_5: write-var gs#138038 <= s_122_4
        fn_state.gs_138038 = s_122_4;
        // N s_122_6: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var el:u8
        let s_123_0: u8 = fn_state.el;
        // D s_123_1: read-var op0:u8
        let s_123_1: u8 = fn_state.op0;
        // D s_123_2: read-var op1:u8
        let s_123_2: u8 = fn_state.op1;
        // D s_123_3: read-var CRn:u8
        let s_123_3: u8 = fn_state.CRn;
        // D s_123_4: read-var op2:u8
        let s_123_4: u8 = fn_state.op2;
        // D s_123_5: read-var CRm:u8
        let s_123_5: u8 = fn_state.CRm;
        // D s_123_6: read-var t:i
        let s_123_6: i128 = fn_state.t;
        // D s_123_7: read-var t2:i
        let s_123_7: i128 = fn_state.t2;
        // D s_123_8: call VTTBR_EL2_SysRegWrite128_3ec09d9d33270957(s_123_0, s_123_1, s_123_2, s_123_3, s_123_4, s_123_5, s_123_6, s_123_7)
        let s_123_8: () = VTTBR_EL2_SysRegWrite128_3ec09d9d33270957(
            state,
            tracer,
            s_123_0,
            s_123_1,
            s_123_2,
            s_123_3,
            s_123_4,
            s_123_5,
            s_123_6,
            s_123_7,
        );
        // N s_123_9: return
        return;
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var op2:u8
        let s_124_0: u8 = fn_state.op2;
        // D s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 3u16);
        // C s_124_2: const #0u : u8
        let s_124_2: u8 = 0;
        // C s_124_3: cast zx s_124_2 -> bv
        let s_124_3: Bits = Bits::new(s_124_2 as u128, 3u16);
        // D s_124_4: cmp-eq s_124_1 s_124_3
        let s_124_4: bool = ((s_124_1) == (s_124_3));
        // D s_124_5: write-var gs#138037 <= s_124_4
        fn_state.gs_138037 = s_124_4;
        // N s_124_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var op1:u8
        let s_125_0: u8 = fn_state.op1;
        // D s_125_1: cast zx s_125_0 -> bv
        let s_125_1: Bits = Bits::new(s_125_0 as u128, 3u16);
        // C s_125_2: const #4u : u8
        let s_125_2: u8 = 4;
        // C s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 3u16);
        // D s_125_4: cmp-eq s_125_1 s_125_3
        let s_125_4: bool = ((s_125_1) == (s_125_3));
        // D s_125_5: write-var gs#138036 <= s_125_4
        fn_state.gs_138036 = s_125_4;
        // N s_125_6: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var op0:u8
        let s_126_0: u8 = fn_state.op0;
        // D s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 2u16);
        // C s_126_2: const #3u : u8
        let s_126_2: u8 = 3;
        // C s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 2u16);
        // D s_126_4: cmp-eq s_126_1 s_126_3
        let s_126_4: bool = ((s_126_1) == (s_126_3));
        // D s_126_5: write-var gs#138035 <= s_126_4
        fn_state.gs_138035 = s_126_4;
        // N s_126_6: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var CRn:u8
        let s_127_0: u8 = fn_state.CRn;
        // D s_127_1: cast zx s_127_0 -> bv
        let s_127_1: Bits = Bits::new(s_127_0 as u128, 4u16);
        // C s_127_2: const #2u : u8
        let s_127_2: u8 = 2;
        // C s_127_3: cast zx s_127_2 -> bv
        let s_127_3: Bits = Bits::new(s_127_2 as u128, 4u16);
        // D s_127_4: cmp-eq s_127_1 s_127_3
        let s_127_4: bool = ((s_127_1) == (s_127_3));
        // D s_127_5: write-var gs#138034 <= s_127_4
        fn_state.gs_138034 = s_127_4;
        // N s_127_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var el:u8
        let s_128_0: u8 = fn_state.el;
        // D s_128_1: read-var op0:u8
        let s_128_1: u8 = fn_state.op0;
        // D s_128_2: read-var op1:u8
        let s_128_2: u8 = fn_state.op1;
        // D s_128_3: read-var CRn:u8
        let s_128_3: u8 = fn_state.CRn;
        // D s_128_4: read-var op2:u8
        let s_128_4: u8 = fn_state.op2;
        // D s_128_5: read-var CRm:u8
        let s_128_5: u8 = fn_state.CRm;
        // D s_128_6: read-var t:i
        let s_128_6: i128 = fn_state.t;
        // D s_128_7: read-var t2:i
        let s_128_7: i128 = fn_state.t2;
        // D s_128_8: call TTBR0_EL1_SysRegWrite128_66ee61de597f133e(s_128_0, s_128_1, s_128_2, s_128_3, s_128_4, s_128_5, s_128_6, s_128_7)
        let s_128_8: () = TTBR0_EL1_SysRegWrite128_66ee61de597f133e(
            state,
            tracer,
            s_128_0,
            s_128_1,
            s_128_2,
            s_128_3,
            s_128_4,
            s_128_5,
            s_128_6,
            s_128_7,
        );
        // N s_128_9: return
        return;
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var op2:u8
        let s_129_0: u8 = fn_state.op2;
        // D s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 3u16);
        // C s_129_2: const #0u : u8
        let s_129_2: u8 = 0;
        // C s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 3u16);
        // D s_129_4: cmp-eq s_129_1 s_129_3
        let s_129_4: bool = ((s_129_1) == (s_129_3));
        // D s_129_5: write-var gs#138033 <= s_129_4
        fn_state.gs_138033 = s_129_4;
        // N s_129_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var op1:u8
        let s_130_0: u8 = fn_state.op1;
        // D s_130_1: cast zx s_130_0 -> bv
        let s_130_1: Bits = Bits::new(s_130_0 as u128, 3u16);
        // C s_130_2: const #0u : u8
        let s_130_2: u8 = 0;
        // C s_130_3: cast zx s_130_2 -> bv
        let s_130_3: Bits = Bits::new(s_130_2 as u128, 3u16);
        // D s_130_4: cmp-eq s_130_1 s_130_3
        let s_130_4: bool = ((s_130_1) == (s_130_3));
        // D s_130_5: write-var gs#138032 <= s_130_4
        fn_state.gs_138032 = s_130_4;
        // N s_130_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var op0:u8
        let s_131_0: u8 = fn_state.op0;
        // D s_131_1: cast zx s_131_0 -> bv
        let s_131_1: Bits = Bits::new(s_131_0 as u128, 2u16);
        // C s_131_2: const #3u : u8
        let s_131_2: u8 = 3;
        // C s_131_3: cast zx s_131_2 -> bv
        let s_131_3: Bits = Bits::new(s_131_2 as u128, 2u16);
        // D s_131_4: cmp-eq s_131_1 s_131_3
        let s_131_4: bool = ((s_131_1) == (s_131_3));
        // D s_131_5: write-var gs#138031 <= s_131_4
        fn_state.gs_138031 = s_131_4;
        // N s_131_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var CRn:u8
        let s_132_0: u8 = fn_state.CRn;
        // D s_132_1: cast zx s_132_0 -> bv
        let s_132_1: Bits = Bits::new(s_132_0 as u128, 4u16);
        // C s_132_2: const #2u : u8
        let s_132_2: u8 = 2;
        // C s_132_3: cast zx s_132_2 -> bv
        let s_132_3: Bits = Bits::new(s_132_2 as u128, 4u16);
        // D s_132_4: cmp-eq s_132_1 s_132_3
        let s_132_4: bool = ((s_132_1) == (s_132_3));
        // D s_132_5: write-var gs#138030 <= s_132_4
        fn_state.gs_138030 = s_132_4;
        // N s_132_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var el:u8
        let s_133_0: u8 = fn_state.el;
        // D s_133_1: read-var op0:u8
        let s_133_1: u8 = fn_state.op0;
        // D s_133_2: read-var op1:u8
        let s_133_2: u8 = fn_state.op1;
        // D s_133_3: read-var CRn:u8
        let s_133_3: u8 = fn_state.CRn;
        // D s_133_4: read-var op2:u8
        let s_133_4: u8 = fn_state.op2;
        // D s_133_5: read-var CRm:u8
        let s_133_5: u8 = fn_state.CRm;
        // D s_133_6: read-var t:i
        let s_133_6: i128 = fn_state.t;
        // D s_133_7: read-var t2:i
        let s_133_7: i128 = fn_state.t2;
        // D s_133_8: call TTBR0_EL2_SysRegWrite128_db720477dd35bd78(s_133_0, s_133_1, s_133_2, s_133_3, s_133_4, s_133_5, s_133_6, s_133_7)
        let s_133_8: () = TTBR0_EL2_SysRegWrite128_db720477dd35bd78(
            state,
            tracer,
            s_133_0,
            s_133_1,
            s_133_2,
            s_133_3,
            s_133_4,
            s_133_5,
            s_133_6,
            s_133_7,
        );
        // N s_133_9: return
        return;
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var op2:u8
        let s_134_0: u8 = fn_state.op2;
        // D s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 3u16);
        // C s_134_2: const #0u : u8
        let s_134_2: u8 = 0;
        // C s_134_3: cast zx s_134_2 -> bv
        let s_134_3: Bits = Bits::new(s_134_2 as u128, 3u16);
        // D s_134_4: cmp-eq s_134_1 s_134_3
        let s_134_4: bool = ((s_134_1) == (s_134_3));
        // D s_134_5: write-var gs#138029 <= s_134_4
        fn_state.gs_138029 = s_134_4;
        // N s_134_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var op1:u8
        let s_135_0: u8 = fn_state.op1;
        // D s_135_1: cast zx s_135_0 -> bv
        let s_135_1: Bits = Bits::new(s_135_0 as u128, 3u16);
        // C s_135_2: const #4u : u8
        let s_135_2: u8 = 4;
        // C s_135_3: cast zx s_135_2 -> bv
        let s_135_3: Bits = Bits::new(s_135_2 as u128, 3u16);
        // D s_135_4: cmp-eq s_135_1 s_135_3
        let s_135_4: bool = ((s_135_1) == (s_135_3));
        // D s_135_5: write-var gs#138028 <= s_135_4
        fn_state.gs_138028 = s_135_4;
        // N s_135_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var op0:u8
        let s_136_0: u8 = fn_state.op0;
        // D s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 2u16);
        // C s_136_2: const #3u : u8
        let s_136_2: u8 = 3;
        // C s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 2u16);
        // D s_136_4: cmp-eq s_136_1 s_136_3
        let s_136_4: bool = ((s_136_1) == (s_136_3));
        // D s_136_5: write-var gs#138027 <= s_136_4
        fn_state.gs_138027 = s_136_4;
        // N s_136_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var CRn:u8
        let s_137_0: u8 = fn_state.CRn;
        // D s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 4u16);
        // C s_137_2: const #2u : u8
        let s_137_2: u8 = 2;
        // C s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 4u16);
        // D s_137_4: cmp-eq s_137_1 s_137_3
        let s_137_4: bool = ((s_137_1) == (s_137_3));
        // D s_137_5: write-var gs#138026 <= s_137_4
        fn_state.gs_138026 = s_137_4;
        // N s_137_6: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var el:u8
        let s_138_0: u8 = fn_state.el;
        // D s_138_1: read-var op0:u8
        let s_138_1: u8 = fn_state.op0;
        // D s_138_2: read-var op1:u8
        let s_138_2: u8 = fn_state.op1;
        // D s_138_3: read-var CRn:u8
        let s_138_3: u8 = fn_state.CRn;
        // D s_138_4: read-var op2:u8
        let s_138_4: u8 = fn_state.op2;
        // D s_138_5: read-var CRm:u8
        let s_138_5: u8 = fn_state.CRm;
        // D s_138_6: read-var t:i
        let s_138_6: i128 = fn_state.t;
        // D s_138_7: read-var t2:i
        let s_138_7: i128 = fn_state.t2;
        // D s_138_8: call TTBR1_EL1_SysRegWrite128_d20c29029eb4ed94(s_138_0, s_138_1, s_138_2, s_138_3, s_138_4, s_138_5, s_138_6, s_138_7)
        let s_138_8: () = TTBR1_EL1_SysRegWrite128_d20c29029eb4ed94(
            state,
            tracer,
            s_138_0,
            s_138_1,
            s_138_2,
            s_138_3,
            s_138_4,
            s_138_5,
            s_138_6,
            s_138_7,
        );
        // N s_138_9: return
        return;
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var op2:u8
        let s_139_0: u8 = fn_state.op2;
        // D s_139_1: cast zx s_139_0 -> bv
        let s_139_1: Bits = Bits::new(s_139_0 as u128, 3u16);
        // C s_139_2: const #1u : u8
        let s_139_2: u8 = 1;
        // C s_139_3: cast zx s_139_2 -> bv
        let s_139_3: Bits = Bits::new(s_139_2 as u128, 3u16);
        // D s_139_4: cmp-eq s_139_1 s_139_3
        let s_139_4: bool = ((s_139_1) == (s_139_3));
        // D s_139_5: write-var gs#138025 <= s_139_4
        fn_state.gs_138025 = s_139_4;
        // N s_139_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var op1:u8
        let s_140_0: u8 = fn_state.op1;
        // D s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 3u16);
        // C s_140_2: const #5u : u8
        let s_140_2: u8 = 5;
        // C s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 3u16);
        // D s_140_4: cmp-eq s_140_1 s_140_3
        let s_140_4: bool = ((s_140_1) == (s_140_3));
        // D s_140_5: write-var gs#138024 <= s_140_4
        fn_state.gs_138024 = s_140_4;
        // N s_140_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var op0:u8
        let s_141_0: u8 = fn_state.op0;
        // D s_141_1: cast zx s_141_0 -> bv
        let s_141_1: Bits = Bits::new(s_141_0 as u128, 2u16);
        // C s_141_2: const #3u : u8
        let s_141_2: u8 = 3;
        // C s_141_3: cast zx s_141_2 -> bv
        let s_141_3: Bits = Bits::new(s_141_2 as u128, 2u16);
        // D s_141_4: cmp-eq s_141_1 s_141_3
        let s_141_4: bool = ((s_141_1) == (s_141_3));
        // D s_141_5: write-var gs#138023 <= s_141_4
        fn_state.gs_138023 = s_141_4;
        // N s_141_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var CRn:u8
        let s_142_0: u8 = fn_state.CRn;
        // D s_142_1: cast zx s_142_0 -> bv
        let s_142_1: Bits = Bits::new(s_142_0 as u128, 4u16);
        // C s_142_2: const #2u : u8
        let s_142_2: u8 = 2;
        // C s_142_3: cast zx s_142_2 -> bv
        let s_142_3: Bits = Bits::new(s_142_2 as u128, 4u16);
        // D s_142_4: cmp-eq s_142_1 s_142_3
        let s_142_4: bool = ((s_142_1) == (s_142_3));
        // D s_142_5: write-var gs#138022 <= s_142_4
        fn_state.gs_138022 = s_142_4;
        // N s_142_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var el:u8
        let s_143_0: u8 = fn_state.el;
        // D s_143_1: read-var op0:u8
        let s_143_1: u8 = fn_state.op0;
        // D s_143_2: read-var op1:u8
        let s_143_2: u8 = fn_state.op1;
        // D s_143_3: read-var CRn:u8
        let s_143_3: u8 = fn_state.CRn;
        // D s_143_4: read-var op2:u8
        let s_143_4: u8 = fn_state.op2;
        // D s_143_5: read-var CRm:u8
        let s_143_5: u8 = fn_state.CRm;
        // D s_143_6: read-var t:i
        let s_143_6: i128 = fn_state.t;
        // D s_143_7: read-var t2:i
        let s_143_7: i128 = fn_state.t2;
        // D s_143_8: call TTBR1_EL2_SysRegWrite128_4bfd1913f2c26b7d(s_143_0, s_143_1, s_143_2, s_143_3, s_143_4, s_143_5, s_143_6, s_143_7)
        let s_143_8: () = TTBR1_EL2_SysRegWrite128_4bfd1913f2c26b7d(
            state,
            tracer,
            s_143_0,
            s_143_1,
            s_143_2,
            s_143_3,
            s_143_4,
            s_143_5,
            s_143_6,
            s_143_7,
        );
        // N s_143_9: return
        return;
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var op2:u8
        let s_144_0: u8 = fn_state.op2;
        // D s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 3u16);
        // C s_144_2: const #1u : u8
        let s_144_2: u8 = 1;
        // C s_144_3: cast zx s_144_2 -> bv
        let s_144_3: Bits = Bits::new(s_144_2 as u128, 3u16);
        // D s_144_4: cmp-eq s_144_1 s_144_3
        let s_144_4: bool = ((s_144_1) == (s_144_3));
        // D s_144_5: write-var gs#138021 <= s_144_4
        fn_state.gs_138021 = s_144_4;
        // N s_144_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var op1:u8
        let s_145_0: u8 = fn_state.op1;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 3u16);
        // C s_145_2: const #0u : u8
        let s_145_2: u8 = 0;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 3u16);
        // D s_145_4: cmp-eq s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) == (s_145_3));
        // D s_145_5: write-var gs#138020 <= s_145_4
        fn_state.gs_138020 = s_145_4;
        // N s_145_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var op0:u8
        let s_146_0: u8 = fn_state.op0;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 2u16);
        // C s_146_2: const #3u : u8
        let s_146_2: u8 = 3;
        // C s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 2u16);
        // D s_146_4: cmp-eq s_146_1 s_146_3
        let s_146_4: bool = ((s_146_1) == (s_146_3));
        // D s_146_5: write-var gs#138019 <= s_146_4
        fn_state.gs_138019 = s_146_4;
        // N s_146_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var CRn:u8
        let s_147_0: u8 = fn_state.CRn;
        // D s_147_1: cast zx s_147_0 -> bv
        let s_147_1: Bits = Bits::new(s_147_0 as u128, 4u16);
        // C s_147_2: const #2u : u8
        let s_147_2: u8 = 2;
        // C s_147_3: cast zx s_147_2 -> bv
        let s_147_3: Bits = Bits::new(s_147_2 as u128, 4u16);
        // D s_147_4: cmp-eq s_147_1 s_147_3
        let s_147_4: bool = ((s_147_1) == (s_147_3));
        // D s_147_5: write-var gs#138018 <= s_147_4
        fn_state.gs_138018 = s_147_4;
        // N s_147_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var el:u8
        let s_148_0: u8 = fn_state.el;
        // D s_148_1: read-var op0:u8
        let s_148_1: u8 = fn_state.op0;
        // D s_148_2: read-var op1:u8
        let s_148_2: u8 = fn_state.op1;
        // D s_148_3: read-var CRn:u8
        let s_148_3: u8 = fn_state.CRn;
        // D s_148_4: read-var op2:u8
        let s_148_4: u8 = fn_state.op2;
        // D s_148_5: read-var CRm:u8
        let s_148_5: u8 = fn_state.CRm;
        // D s_148_6: read-var t:i
        let s_148_6: i128 = fn_state.t;
        // D s_148_7: read-var t2:i
        let s_148_7: i128 = fn_state.t2;
        // D s_148_8: call RCWSMASK_EL1_SysRegWrite128_d70b97b0479af313(s_148_0, s_148_1, s_148_2, s_148_3, s_148_4, s_148_5, s_148_6, s_148_7)
        let s_148_8: () = RCWSMASK_EL1_SysRegWrite128_d70b97b0479af313(
            state,
            tracer,
            s_148_0,
            s_148_1,
            s_148_2,
            s_148_3,
            s_148_4,
            s_148_5,
            s_148_6,
            s_148_7,
        );
        // N s_148_9: return
        return;
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var op2:u8
        let s_149_0: u8 = fn_state.op2;
        // D s_149_1: cast zx s_149_0 -> bv
        let s_149_1: Bits = Bits::new(s_149_0 as u128, 3u16);
        // C s_149_2: const #3u : u8
        let s_149_2: u8 = 3;
        // C s_149_3: cast zx s_149_2 -> bv
        let s_149_3: Bits = Bits::new(s_149_2 as u128, 3u16);
        // D s_149_4: cmp-eq s_149_1 s_149_3
        let s_149_4: bool = ((s_149_1) == (s_149_3));
        // D s_149_5: write-var gs#138017 <= s_149_4
        fn_state.gs_138017 = s_149_4;
        // N s_149_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var op1:u8
        let s_150_0: u8 = fn_state.op1;
        // D s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 3u16);
        // C s_150_2: const #0u : u8
        let s_150_2: u8 = 0;
        // C s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 3u16);
        // D s_150_4: cmp-eq s_150_1 s_150_3
        let s_150_4: bool = ((s_150_1) == (s_150_3));
        // D s_150_5: write-var gs#138016 <= s_150_4
        fn_state.gs_138016 = s_150_4;
        // N s_150_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var op0:u8
        let s_151_0: u8 = fn_state.op0;
        // D s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 2u16);
        // C s_151_2: const #3u : u8
        let s_151_2: u8 = 3;
        // C s_151_3: cast zx s_151_2 -> bv
        let s_151_3: Bits = Bits::new(s_151_2 as u128, 2u16);
        // D s_151_4: cmp-eq s_151_1 s_151_3
        let s_151_4: bool = ((s_151_1) == (s_151_3));
        // D s_151_5: write-var gs#138015 <= s_151_4
        fn_state.gs_138015 = s_151_4;
        // N s_151_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var CRn:u8
        let s_152_0: u8 = fn_state.CRn;
        // D s_152_1: cast zx s_152_0 -> bv
        let s_152_1: Bits = Bits::new(s_152_0 as u128, 4u16);
        // C s_152_2: const #13u : u8
        let s_152_2: u8 = 13;
        // C s_152_3: cast zx s_152_2 -> bv
        let s_152_3: Bits = Bits::new(s_152_2 as u128, 4u16);
        // D s_152_4: cmp-eq s_152_1 s_152_3
        let s_152_4: bool = ((s_152_1) == (s_152_3));
        // D s_152_5: write-var gs#138014 <= s_152_4
        fn_state.gs_138014 = s_152_4;
        // N s_152_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var el:u8
        let s_153_0: u8 = fn_state.el;
        // D s_153_1: read-var op0:u8
        let s_153_1: u8 = fn_state.op0;
        // D s_153_2: read-var op1:u8
        let s_153_2: u8 = fn_state.op1;
        // D s_153_3: read-var CRn:u8
        let s_153_3: u8 = fn_state.CRn;
        // D s_153_4: read-var op2:u8
        let s_153_4: u8 = fn_state.op2;
        // D s_153_5: read-var CRm:u8
        let s_153_5: u8 = fn_state.CRm;
        // D s_153_6: read-var t:i
        let s_153_6: i128 = fn_state.t;
        // D s_153_7: read-var t2:i
        let s_153_7: i128 = fn_state.t2;
        // D s_153_8: call PAR_EL1_SysRegWrite128_e67f3244d494abe6(s_153_0, s_153_1, s_153_2, s_153_3, s_153_4, s_153_5, s_153_6, s_153_7)
        let s_153_8: () = PAR_EL1_SysRegWrite128_e67f3244d494abe6(
            state,
            tracer,
            s_153_0,
            s_153_1,
            s_153_2,
            s_153_3,
            s_153_4,
            s_153_5,
            s_153_6,
            s_153_7,
        );
        // N s_153_9: return
        return;
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var op2:u8
        let s_154_0: u8 = fn_state.op2;
        // D s_154_1: cast zx s_154_0 -> bv
        let s_154_1: Bits = Bits::new(s_154_0 as u128, 3u16);
        // C s_154_2: const #0u : u8
        let s_154_2: u8 = 0;
        // C s_154_3: cast zx s_154_2 -> bv
        let s_154_3: Bits = Bits::new(s_154_2 as u128, 3u16);
        // D s_154_4: cmp-eq s_154_1 s_154_3
        let s_154_4: bool = ((s_154_1) == (s_154_3));
        // D s_154_5: write-var gs#138013 <= s_154_4
        fn_state.gs_138013 = s_154_4;
        // N s_154_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var op1:u8
        let s_155_0: u8 = fn_state.op1;
        // D s_155_1: cast zx s_155_0 -> bv
        let s_155_1: Bits = Bits::new(s_155_0 as u128, 3u16);
        // C s_155_2: const #0u : u8
        let s_155_2: u8 = 0;
        // C s_155_3: cast zx s_155_2 -> bv
        let s_155_3: Bits = Bits::new(s_155_2 as u128, 3u16);
        // D s_155_4: cmp-eq s_155_1 s_155_3
        let s_155_4: bool = ((s_155_1) == (s_155_3));
        // D s_155_5: write-var gs#138012 <= s_155_4
        fn_state.gs_138012 = s_155_4;
        // N s_155_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var op0:u8
        let s_156_0: u8 = fn_state.op0;
        // D s_156_1: cast zx s_156_0 -> bv
        let s_156_1: Bits = Bits::new(s_156_0 as u128, 2u16);
        // C s_156_2: const #3u : u8
        let s_156_2: u8 = 3;
        // C s_156_3: cast zx s_156_2 -> bv
        let s_156_3: Bits = Bits::new(s_156_2 as u128, 2u16);
        // D s_156_4: cmp-eq s_156_1 s_156_3
        let s_156_4: bool = ((s_156_1) == (s_156_3));
        // D s_156_5: write-var gs#138011 <= s_156_4
        fn_state.gs_138011 = s_156_4;
        // N s_156_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var CRn:u8
        let s_157_0: u8 = fn_state.CRn;
        // D s_157_1: cast zx s_157_0 -> bv
        let s_157_1: Bits = Bits::new(s_157_0 as u128, 4u16);
        // C s_157_2: const #7u : u8
        let s_157_2: u8 = 7;
        // C s_157_3: cast zx s_157_2 -> bv
        let s_157_3: Bits = Bits::new(s_157_2 as u128, 4u16);
        // D s_157_4: cmp-eq s_157_1 s_157_3
        let s_157_4: bool = ((s_157_1) == (s_157_3));
        // D s_157_5: write-var gs#138010 <= s_157_4
        fn_state.gs_138010 = s_157_4;
        // N s_157_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}

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
use AArch32_WriteModeByInstr::*;
use common::*;
pub fn execute_aarch32_instrs_CPS_OpT_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    affectA: bool,
    affectF: bool,
    affectI: bool,
    changemode: bool,
    disable: bool,
    enable: bool,
    mode: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        affectA: bool,
        affectF: bool,
        affectI: bool,
        changemode: bool,
        disable: bool,
        enable: bool,
        mode: u8,
    }
    let fn_state = FunctionState {
        affectA,
        affectF,
        affectI,
        changemode,
        disable,
        enable,
        mode,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-ne s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) != (s_0_5));
        // N s_0_7: branch s_0_6 b2 b1
        if s_0_6 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var enable:u8
        let s_2_0: bool = fn_state.enable;
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
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var disable:u8
        let s_4_0: bool = fn_state.disable;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var changemode:u8
        let s_6_0: bool = fn_state.changemode;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var mode:u8
        let s_8_0: u8 = fn_state.mode;
        // D s_8_1: call AArch32_WriteModeByInstr(s_8_0)
        let s_8_1: () = AArch32_WriteModeByInstr(state, tracer, s_8_0);
        // N s_8_2: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var affectA:u8
        let s_9_0: bool = fn_state.affectA;
        // N s_9_1: branch s_9_0 b18 b10
        if s_9_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var affectI:u8
        let s_11_0: bool = fn_state.affectI;
        // N s_11_1: branch s_11_0 b17 b12
        if s_11_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var affectF:u8
        let s_13_0: bool = fn_state.affectF;
        // N s_13_1: branch s_13_0 b16 b14
        if s_13_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // C s_16_1: const #16977u : u32
        let s_16_1: u32 = 16977;
        // N s_16_2: write-reg s_16_1 <= s_16_0
        let s_16_2: () = {
            state.write_register::<bool>(s_16_1 as isize, s_16_0);
            tracer.write_register(s_16_1 as isize, s_16_0);
        };
        // N s_16_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // C s_17_1: const #16979u : u32
        let s_17_1: u32 = 16979;
        // N s_17_2: write-reg s_17_1 <= s_17_0
        let s_17_2: () = {
            state.write_register::<bool>(s_17_1 as isize, s_17_0);
            tracer.write_register(s_17_1 as isize, s_17_0);
        };
        // N s_17_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // C s_18_1: const #16968u : u32
        let s_18_1: u32 = 16968;
        // N s_18_2: write-reg s_18_1 <= s_18_0
        let s_18_2: () = {
            state.write_register::<bool>(s_18_1 as isize, s_18_0);
            tracer.write_register(s_18_1 as isize, s_18_0);
        };
        // N s_18_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var affectA:u8
        let s_19_0: bool = fn_state.affectA;
        // N s_19_1: branch s_19_0 b28 b20
        if s_19_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var affectI:u8
        let s_21_0: bool = fn_state.affectI;
        // N s_21_1: branch s_21_0 b27 b22
        if s_21_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var affectF:u8
        let s_23_0: bool = fn_state.affectF;
        // N s_23_1: branch s_23_0 b26 b24
        if s_23_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // C s_26_1: const #16977u : u32
        let s_26_1: u32 = 16977;
        // N s_26_2: write-reg s_26_1 <= s_26_0
        let s_26_2: () = {
            state.write_register::<bool>(s_26_1 as isize, s_26_0);
            tracer.write_register(s_26_1 as isize, s_26_0);
        };
        // N s_26_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // C s_27_1: const #16979u : u32
        let s_27_1: u32 = 16979;
        // N s_27_2: write-reg s_27_1 <= s_27_0
        let s_27_2: () = {
            state.write_register::<bool>(s_27_1 as isize, s_27_0);
            tracer.write_register(s_27_1 as isize, s_27_0);
        };
        // N s_27_3: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // C s_28_1: const #16968u : u32
        let s_28_1: u32 = 16968;
        // N s_28_2: write-reg s_28_1 <= s_28_0
        let s_28_2: () = {
            state.write_register::<bool>(s_28_1 as isize, s_28_0);
            tracer.write_register(s_28_1 as isize, s_28_0);
        };
        // N s_28_3: jump b21
        return block_21(state, tracer, fn_state);
    }
}

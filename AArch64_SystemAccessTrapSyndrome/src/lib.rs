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
use Bit::*;
use ConditionSyndrome::*;
use ThisInstr::*;
use ExceptionSyndrome::*;
use Unreachable::*;
use common::*;
pub fn AArch64_SystemAccessTrapSyndrome<T: Tracer>(
    state: &mut State,
    tracer: &T,
    instr_in: u32,
    ec: i128,
) -> ProductTypeb7f99f96751e17c4 {
    #[derive(Default)]
    struct FunctionState {
        except: ProductTypeb7f99f96751e17c4,
        instr_in: u32,
        ec: i128,
    }
    let fn_state = FunctionState {
        instr_in,
        ec,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_0_0: read-var ec:i
        let s_0_0: i128 = fn_state.ec;
        // C s_0_1: const #0s : i
        let s_0_1: i128 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
        // S s_1_1: call ExceptionSyndrome(s_1_0)
        let s_1_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_1_0);
        // D s_1_2: write-var except <= s_1_1
        fn_state.except = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_2_0: read-var except:struct
        let s_2_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_3_0: read-var ec:i
        let s_3_0: i128 = fn_state.ec;
        // C s_3_1: const #7s : i
        let s_3_1: i128 = 7;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_4_0: const #7u : u32
        let s_4_0: u32 = 7;
        // S s_4_1: call ExceptionSyndrome(s_4_0)
        let s_4_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_4_0);
        // D s_4_2: write-var except <= s_4_1
        fn_state.except = s_4_1;
        // C s_4_3: const #() : ()
        let s_4_3: () = ();
        // S s_4_4: call ConditionSyndrome(s_4_3)
        let s_4_4: u8 = ConditionSyndrome(state, tracer, s_4_3);
        // D s_4_5: read-var except:struct
        let s_4_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_4_6: write-var except <= s_4_5
        fn_state.except = s_4_5;
        // N s_4_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_5_0: read-var ec:i
        let s_5_0: i128 = fn_state.ec;
        // C s_5_1: const #20s : i
        let s_5_1: i128 = 20;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_6_0: const #39u : u32
        let s_6_0: u32 = 39;
        // S s_6_1: call ExceptionSyndrome(s_6_0)
        let s_6_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_6_0);
        // D s_6_2: write-var except <= s_6_1
        fn_state.except = s_6_1;
        // C s_6_3: const #() : ()
        let s_6_3: () = ();
        // S s_6_4: call ThisInstr(s_6_3)
        let s_6_4: u32 = ThisInstr(state, tracer, s_6_3);
        // D s_6_5: read-var except:struct
        let s_6_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_6: write-var except <= s_6_5
        fn_state.except = s_6_5;
        // D s_6_7: read-var except:struct
        let s_6_7: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_8: write-var except <= s_6_7
        fn_state.except = s_6_7;
        // D s_6_9: read-var except:struct
        let s_6_9: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_10: write-var except <= s_6_9
        fn_state.except = s_6_9;
        // D s_6_11: read-var except:struct
        let s_6_11: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_12: write-var except <= s_6_11
        fn_state.except = s_6_11;
        // D s_6_13: read-var except:struct
        let s_6_13: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_14: write-var except <= s_6_13
        fn_state.except = s_6_13;
        // D s_6_15: read-var except:struct
        let s_6_15: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_16: write-var except <= s_6_15
        fn_state.except = s_6_15;
        // C s_6_17: const #21s : i
        let s_6_17: i128 = 21;
        // S s_6_18: cast zx s_6_4 -> bv
        let s_6_18: Bits = Bits::new(s_6_4 as u128, 32u16);
        // C s_6_19: const #1u : u64
        let s_6_19: u64 = 1;
        // D s_6_20: bit-extract s_6_18 s_6_17 s_6_19
        let s_6_20: Bits = (Bits::new(
            ((s_6_18) >> (s_6_17)).value(),
            u16::try_from(s_6_19).unwrap(),
        ));
        // D s_6_21: cast reint s_6_20 -> u8
        let s_6_21: bool = ((s_6_20.value()) != 0);
        // C s_6_22: const #0s : i
        let s_6_22: i128 = 0;
        // C s_6_23: const #0u : u64
        let s_6_23: u64 = 0;
        // D s_6_24: cast zx s_6_21 -> u64
        let s_6_24: u64 = (s_6_21 as u64);
        // C s_6_25: const #1u : u64
        let s_6_25: u64 = 1;
        // D s_6_26: and s_6_24 s_6_25
        let s_6_26: u64 = ((s_6_24) & (s_6_25));
        // D s_6_27: cmp-eq s_6_26 s_6_25
        let s_6_27: bool = ((s_6_26) == (s_6_25));
        // D s_6_28: lsl s_6_24 s_6_22
        let s_6_28: u64 = s_6_24 << s_6_22;
        // D s_6_29: or s_6_23 s_6_28
        let s_6_29: u64 = ((s_6_23) | (s_6_28));
        // D s_6_30: cmpl s_6_28
        let s_6_30: u64 = !s_6_28;
        // D s_6_31: and s_6_23 s_6_30
        let s_6_31: u64 = ((s_6_23) & (s_6_30));
        // D s_6_32: select s_6_27 s_6_29 s_6_31
        let s_6_32: u64 = if s_6_27 { s_6_29 } else { s_6_31 };
        // D s_6_33: cast trunc s_6_32 -> u8
        let s_6_33: bool = ((s_6_32) != 0);
        // D s_6_34: call Bit(s_6_33)
        let s_6_34: bool = Bit(state, tracer, s_6_33);
        // D s_6_35: read-var except:struct
        let s_6_35: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_6_36: write-var except <= s_6_35
        fn_state.except = s_6_35;
        // N s_6_37: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_7_0: read-var ec:i
        let s_7_0: i128 = fn_state.ec;
        // C s_7_1: const #24s : i
        let s_7_1: i128 = 24;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_8_0: const #15u : u32
        let s_8_0: u32 = 15;
        // S s_8_1: call ExceptionSyndrome(s_8_0)
        let s_8_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_8_0);
        // D s_8_2: write-var except <= s_8_1
        fn_state.except = s_8_1;
        // C s_8_3: const #() : ()
        let s_8_3: () = ();
        // S s_8_4: call ThisInstr(s_8_3)
        let s_8_4: u32 = ThisInstr(state, tracer, s_8_3);
        // D s_8_5: read-var except:struct
        let s_8_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_6: write-var except <= s_8_5
        fn_state.except = s_8_5;
        // D s_8_7: read-var except:struct
        let s_8_7: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_8: write-var except <= s_8_7
        fn_state.except = s_8_7;
        // D s_8_9: read-var except:struct
        let s_8_9: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_10: write-var except <= s_8_9
        fn_state.except = s_8_9;
        // D s_8_11: read-var except:struct
        let s_8_11: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_12: write-var except <= s_8_11
        fn_state.except = s_8_11;
        // D s_8_13: read-var except:struct
        let s_8_13: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_14: write-var except <= s_8_13
        fn_state.except = s_8_13;
        // D s_8_15: read-var except:struct
        let s_8_15: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_16: write-var except <= s_8_15
        fn_state.except = s_8_15;
        // C s_8_17: const #21s : i
        let s_8_17: i128 = 21;
        // S s_8_18: cast zx s_8_4 -> bv
        let s_8_18: Bits = Bits::new(s_8_4 as u128, 32u16);
        // C s_8_19: const #1u : u64
        let s_8_19: u64 = 1;
        // D s_8_20: bit-extract s_8_18 s_8_17 s_8_19
        let s_8_20: Bits = (Bits::new(
            ((s_8_18) >> (s_8_17)).value(),
            u16::try_from(s_8_19).unwrap(),
        ));
        // D s_8_21: cast reint s_8_20 -> u8
        let s_8_21: bool = ((s_8_20.value()) != 0);
        // C s_8_22: const #0s : i
        let s_8_22: i128 = 0;
        // C s_8_23: const #0u : u64
        let s_8_23: u64 = 0;
        // D s_8_24: cast zx s_8_21 -> u64
        let s_8_24: u64 = (s_8_21 as u64);
        // C s_8_25: const #1u : u64
        let s_8_25: u64 = 1;
        // D s_8_26: and s_8_24 s_8_25
        let s_8_26: u64 = ((s_8_24) & (s_8_25));
        // D s_8_27: cmp-eq s_8_26 s_8_25
        let s_8_27: bool = ((s_8_26) == (s_8_25));
        // D s_8_28: lsl s_8_24 s_8_22
        let s_8_28: u64 = s_8_24 << s_8_22;
        // D s_8_29: or s_8_23 s_8_28
        let s_8_29: u64 = ((s_8_23) | (s_8_28));
        // D s_8_30: cmpl s_8_28
        let s_8_30: u64 = !s_8_28;
        // D s_8_31: and s_8_23 s_8_30
        let s_8_31: u64 = ((s_8_23) & (s_8_30));
        // D s_8_32: select s_8_27 s_8_29 s_8_31
        let s_8_32: u64 = if s_8_27 { s_8_29 } else { s_8_31 };
        // D s_8_33: cast trunc s_8_32 -> u8
        let s_8_33: bool = ((s_8_32) != 0);
        // D s_8_34: call Bit(s_8_33)
        let s_8_34: bool = Bit(state, tracer, s_8_33);
        // D s_8_35: read-var except:struct
        let s_8_35: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_8_36: write-var except <= s_8_35
        fn_state.except = s_8_35;
        // N s_8_37: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_9_0: read-var ec:i
        let s_9_0: i128 = fn_state.ec;
        // C s_9_1: const #25s : i
        let s_9_1: i128 = 25;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b11 b10
        if s_9_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_10_0: const #32u : u32
        let s_10_0: u32 = 32;
        // S s_10_1: call ExceptionSyndrome(s_10_0)
        let s_10_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_10_0,
        );
        // D s_10_2: write-var except <= s_10_1
        fn_state.except = s_10_1;
        // N s_10_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_11_0: read-var ec:i
        let s_11_0: i128 = fn_state.ec;
        // C s_11_1: const #29s : i
        let s_11_1: i128 = 29;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: not s_11_2
        let s_11_3: bool = !s_11_2;
        // N s_11_4: branch s_11_3 b13 b12
        if s_11_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_12_0: const #33u : u32
        let s_12_0: u32 = 33;
        // S s_12_1: call ExceptionSyndrome(s_12_0)
        let s_12_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_12_0,
        );
        // D s_12_2: write-var except <= s_12_1
        fn_state.except = s_12_1;
        // N s_12_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call Unreachable(s_13_0)
        let s_13_1: () = Unreachable(state, tracer, s_13_0);
        // N s_13_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
